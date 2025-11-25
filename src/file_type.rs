use std::fmt;
use std::path::Path;

/// Represents the supported encrypted file types.
/// This enum encodes file type invariants in the type system,
/// eliminating the need for string-based extension comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// GPG encrypted file (.gpg)
    Gpg,
    /// Age encrypted file (.age)
    Age,
}

impl FileType {
    /// Attempts to determine the file type from a file path.
    ///
    /// # Arguments
    /// * `path` - The file path to analyze
    ///
    /// # Returns
    /// * `Some(FileType)` if the file has a supported extension
    /// * `None` if the extension is unsupported or missing
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?;
        match ext {
            "gpg" => Some(FileType::Gpg),
            "age" => Some(FileType::Age),
            _ => None,
        }
    }

    /// Returns the file extension as a string slice.
    /// Useful for compatibility with APIs that expect string extensions.
    pub fn extension(&self) -> &'static str {
        match self {
            FileType::Gpg => "gpg",
            FileType::Age => "age",
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.extension())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_from_path_gpg() {
        let path = PathBuf::from("test.gpg");
        assert_eq!(FileType::from_path(&path), Some(FileType::Gpg));
    }

    #[test]
    fn test_from_path_age() {
        let path = PathBuf::from("test.age");
        assert_eq!(FileType::from_path(&path), Some(FileType::Age));
    }

    #[test]
    fn test_from_path_unsupported() {
        let path = PathBuf::from("test.txt");
        assert_eq!(FileType::from_path(&path), None);
    }

    #[test]
    fn test_extension() {
        assert_eq!(FileType::Gpg.extension(), "gpg");
        assert_eq!(FileType::Age.extension(), "age");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FileType::Gpg), "gpg");
        assert_eq!(format!("{}", FileType::Age), "age");
    }
}
