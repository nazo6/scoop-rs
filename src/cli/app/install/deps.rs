use std::collections::{HashMap, HashSet};

use interface::bucket_app::BucketApp;

pub async fn resolve_deps<'a>(
    app: &BucketApp<'a>,
    apps: &HashMap<String, BucketApp<'a>>,
    deps: &mut HashSet<BucketApp<'a>>,
) -> anyhow::Result<()> {
    let manifest = app.manifest().await?;

    for dep in manifest.depends {
        let dep = apps
            .get(&dep)
            .ok_or_else(|| anyhow::anyhow!("Dependency `{}` not found in any bucket.", dep))?;
        if deps.insert(dep.clone()) {
            Box::pin(resolve_deps(dep, apps, deps)).await?;
        }
    }

    Ok(())
}
