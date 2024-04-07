use crate::{cli::CliResult, interface::bucket::get_buckets};

pub async fn start() -> CliResult {
    println!("List of buckets");
    let buckets = get_buckets()
        .await
        .map_err(|e| format!("Failed to get buckets: {}", e))?;
    let str = buckets
        .iter()
        .map(|b| b.name.as_str())
        .collect::<Vec<&str>>()
        .join("\n");
    println!("{}", str);

    Ok(())
}
