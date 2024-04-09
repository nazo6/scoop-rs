use interface::{
    bucket_app::{BucketApp, BucketsAppsRepository},
    manifest::Manifest,
};

pub async fn resolve<'a>(
    app: &'a BucketApp<'a>,
    apps: &'a BucketsAppsRepository<'a>,
) -> anyhow::Result<Vec<(&'a BucketApp<'a>, Manifest)>> {
    let mut to_install = vec![(app, app.manifest().await?)];
    resolve_inner((app, app.manifest().await?), apps, &mut to_install).await?;
    Ok(to_install)
}

async fn resolve_inner<'a>(
    (app, manifest): (&BucketApp<'a>, Manifest),
    apps: &'a BucketsAppsRepository<'a>,
    to_install: &mut Vec<(&'a BucketApp<'a>, Manifest)>,
) -> anyhow::Result<()> {
    let depends = manifest.depends;
    for depend_name in depends.unwrap_or_default() {
        let dependency = depend_name.get_bucket_app(apps).ok_or_else(|| {
            anyhow::anyhow!("Dependency `{}` not found in any bucket.", depend_name)
        })?;
        if !to_install.iter().any(|(a, _)| a.name == dependency.name) {
            let manifest = app.manifest().await?;
            Box::pin(resolve_inner((dependency, manifest), apps, to_install)).await?;
        }
    }
    Ok(())
}
