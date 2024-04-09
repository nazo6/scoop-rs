use std::fmt::Write;

use clap::Args;
use futures_util::StreamExt as _;
use indicatif::{MultiProgress, ProgressState, ProgressStyle};
use interface::{
    bucket::get_buckets,
    bucket_app::{BucketAppName, BucketsAppsRepository},
    val::CACHE_PATH,
};
use tokio::{fs::File, io::AsyncWriteExt as _};

use crate::cli::CliResult;

mod resolve;

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[clap(required = true)]
    pub apps: Vec<BucketAppName>,
    #[clap(long, default_value_t = false)]
    pub no_hash_check: bool,
}

const DOWNLOAD_CONCURRENCY: usize = 4;

pub async fn start(opts: InstallArgs) -> CliResult {
    let buckets = get_buckets()
        .await
        .map_err(|e| format!("Failed to get buckets: {}", e))?;
    let apps = BucketsAppsRepository::from_buckets(&buckets)
        .await
        .map_err(|e| format!("Failed to get apps: {}", e))?;

    let mut packages = Vec::new();
    for app_name in opts.apps {
        let app = app_name
            .get_bucket_app(&apps)
            .ok_or_else(|| format!("App `{}` not found in any bucket.", app_name))?;
        let to_install = resolve::resolve(app, &apps)
            .await
            .map_err(|e| format!("Failed to resolve dependencies for `{}`: {}", app_name, e))?;
        packages.extend(to_install);
    }

    let m = MultiProgress::new();
    let mut download_futures = Vec::new();

    for (app, manifest) in packages {
        let version = if manifest.version == "nightly" {
            format!("nightly-{}", chrono::Utc::now().format("%Y-%m-%d"))
        } else {
            manifest.version.clone()
        };

        let urls = manifest
            .architecture(interface::manifest::Architecture::Amd64)
            .url;
        let urls = urls.unwrap_or_default();
        let name = app.name.clone();
        let m = m.clone();
        for url in urls.into_iter() {
            let cache_file_name = format!(
                "{}-{}-{}",
                name,
                version,
                sanitize_filename::sanitize(&url.url)
            );
            download_futures.push(tokio::spawn(download_files(
                url.url,
                cache_file_name,
                format!("{}: {}", name, version),
                m.clone(),
            )))
        }
    }
    println!("Downloading {} files...", download_futures.len());
    let stream = futures::stream::iter(download_futures).buffer_unordered(DOWNLOAD_CONCURRENCY);
    let results = stream.collect::<Vec<_>>().await;

    Ok(())
}

async fn download_files(
    url: String,
    name: String,
    show_name: String,
    progress: MultiProgress,
) -> Result<(), anyhow::Error> {
    let mut file = File::create(CACHE_PATH.join(&name)).await?;

    let pb = progress.add(indicatif::ProgressBar::new(100));
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    pb.set_message(show_name.clone());

    let resp = reqwest::get(url).await?;
    let length = resp.content_length();

    pb.set_length(length.unwrap_or(0));

    let mut stream = resp.bytes_stream();

    let mut downloaded = 0;
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    pb.finish_with_message(format!("Downloaded {}", show_name));

    Ok(())
}
