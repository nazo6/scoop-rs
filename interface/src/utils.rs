/// "a.exe"
pub fn get_stem(name: &str) -> (&str, Option<&str>) {
    if let Some((stem, ext)) = name.rsplit_once('.') {
        (stem, Some(ext))
    } else {
        (name, None)
    }
}
