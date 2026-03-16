use std::path::Path;

pub fn open_with_default_app(path: &Path) -> Result<(), std::io::Error> {
    open::that(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
