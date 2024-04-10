use std::{fmt::Write, path::PathBuf};

use futures_util::StreamExt as _;
use indicatif::{MultiProgress, ProgressState, ProgressStyle};
use interface::{bucket_app::BucketApp, dir::CACHE_DIR, manifest::Manifest};
use tokio::{fs::File, io::AsyncWriteExt as _};

const DOWNLOAD_CONCURRENCY: usize = 4;

pub async fn download<'a>(install_apps: &'a [(&'a BucketApp<'a>, Manifest)]) {
    let m = MultiProgress::new();
    let mut download_futures = Vec::new();

    for (app, manifest) in install_apps {
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
        let url_count = urls.len();
        for (i, url) in urls.into_iter().enumerate() {
            let cache_file_name = format!(
                "{}-{}-{}",
                &name,
                &version,
                sanitize_filename::sanitize(&url.url)
            );
            let show_name = if url_count == 1 {
                format!("{} {}", &name, &version)
            } else {
                format!("{} {} ({})", &name, &version, i + 1)
            };
            let m = m.clone();
            download_futures.push(async move {
                download_to_cache(url.url, cache_file_name, show_name, m.clone()).await
            })
        }
    }
    println!("Downloading {} files...", download_futures.len());
    let stream = futures::stream::iter(download_futures).buffer_unordered(DOWNLOAD_CONCURRENCY);
    let _results = stream.collect::<Vec<_>>().await;
}

async fn download_to_cache(
    url: String,
    cache_file_name: String,
    show_name: String,
    progress: MultiProgress,
) -> Result<(), anyhow::Error> {
    let mut file = File::create(CACHE_DIR.join(&cache_file_name)).await?;

    let pb = progress.add(indicatif::ProgressBar::new(100));
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    pb.set_message(show_name.to_string());

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
