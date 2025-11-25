// Library interface for the decryptor
// This exposes the public API for use in fuzz testing and other library consumers

pub mod file_type;

// Re-export commonly used types for convenience
pub use file_type::FileType;
