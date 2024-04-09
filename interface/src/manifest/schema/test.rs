use crate::bucket::get_buckets;

#[tokio::test]
async fn can_parse_all_local_manifest() {
    let buckets = get_buckets().await.expect("Failed to get buckets");
    for bucket in &buckets {
        let Ok(apps) = bucket.apps().await else {
            println!("Failed to get apps for {}", bucket.name);
            continue;
        };
        let len = apps.len();
        for app in apps {
            let _ = app.manifest().await.unwrap_or_else(|e| {
                panic!(
                    "Failed to get manifest for {} in {}:\n\t{}",
                    app.name, bucket.name, e
                )
            });
        }
        println!("Parsed all {} apps in {}", len, bucket.name);
    }
}
