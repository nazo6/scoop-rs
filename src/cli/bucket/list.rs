use interface::bucket::get_buckets;
use tabled::settings::Style;

use crate::cli::CliResult;

pub async fn start() -> CliResult {
    println!("List of buckets");
    let buckets = get_buckets()
        .await
        .map_err(|e| format!("Failed to get buckets: {}", e))?;

    let mut builder = tabled::builder::Builder::default();
    builder.push_record(["Name", "Url"]);

    for bucket in buckets {
        let remote_url = if let Ok(repo) = bucket.repository() {
            repo.find_remote("origin")
                .ok()
                .and_then(|r| r.url().map(|u| u.to_string()))
                .unwrap_or_else(|| "No remote found".to_string())
        } else {
            "Failed to get remote".to_string()
        };
        builder.push_record([bucket.name.as_str(), &remote_url]);
    }

    let table = builder.build().with(Style::rounded()).to_string();

    println!("{}", table);

    Ok(())
}
