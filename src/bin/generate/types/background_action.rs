use types::*;


/// A background image.
pub struct BackgroundAction {
    /// The path to the background image.
    pub path: std::path::PathBuf,
}


impl Action for BackgroundAction {
    /// Converts a string to a background description.
    ///
    /// The string must be a path.
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self { path: std::path::Path::new(s).to_path_buf() })
    }
}
