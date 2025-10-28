use std::fs;
use std::path::Path;
use anyhow::Result;

/// Cross-platform file permissions and deletion utilities
pub struct PlatformUtils;

impl PlatformUtils {
    /// Check if we have write permissions to a path (cross-platform)
    pub fn can_delete<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();
        
        if !path.exists() {
            return false;
        }

        // On Windows, check if the file/directory is read-only
        #[cfg(windows)]
        {
            if let Ok(metadata) = fs::metadata(path) {
                use std::os::windows::fs::MetadataExt;
                let attributes = metadata.file_attributes();
                const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
                return (attributes & FILE_ATTRIBUTE_READONLY) == 0;
            }
            false
        }

        // On Unix-like systems, check if we have write permission
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(path) {
                let mode = metadata.permissions().mode();
                // Check if owner has write permission (simplified)
                return (mode & 0o200) != 0;
            }
            false
        }

        // Fallback for other platforms
        #[cfg(not(any(windows, unix)))]
        {
            // Just try to open the parent directory for writing as a test
            if path.is_file() {
                if let Some(parent) = path.parent() {
                    return parent.exists();
                }
            }
            path.exists()
        }
    }

    /// Safely delete a file or directory with proper error handling
    pub fn safe_delete<P: AsRef<Path>>(path: P, is_directory: bool) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(anyhow::anyhow!("Path '{}' does not exist", path.display()));
        }

        // Check permissions before attempting deletion
        if !Self::can_delete(path) {
            return Err(anyhow::anyhow!(
                "Insufficient permissions to delete '{}'", 
                path.display()
            ));
        }

        // On Windows, remove read-only attribute if present
        #[cfg(windows)]
        {
            if let Ok(metadata) = fs::metadata(path) {
                use std::os::windows::fs::MetadataExt;
                let attributes = metadata.file_attributes();
                const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
                
                if (attributes & FILE_ATTRIBUTE_READONLY) != 0 {
                    // Try to remove read-only attribute
                    let mut perms = metadata.permissions();
                    perms.set_readonly(false);
                    let _ = fs::set_permissions(path, perms);
                }
            }
        }

        // Perform the actual deletion
        if is_directory {
            fs::remove_dir_all(path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to delete directory '{}': {}", 
                    path.display(), 
                    e
                )
            })?;
        } else {
            fs::remove_file(path).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to delete file '{}': {}", 
                    path.display(), 
                    e
                )
            })?;
        }

        Ok(())
    }

    /// Get a user-friendly error message for common file operation errors
    #[allow(dead_code)] // May be used in future for better error handling
    pub fn friendly_error_message(error: &std::io::Error) -> String {
        match error.kind() {
            std::io::ErrorKind::PermissionDenied => {
                "Permission denied. You may need administrator privileges.".to_string()
            }
            std::io::ErrorKind::NotFound => {
                "File or directory not found.".to_string()
            }
            std::io::ErrorKind::DirectoryNotEmpty => {
                "Directory is not empty and cannot be deleted.".to_string()
            }
            _ => {
                format!("Operation failed: {}", error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;

    #[test]
    fn test_can_delete_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();
        
        assert!(PlatformUtils::can_delete(&file_path));
    }

    #[test]
    fn test_can_delete_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");
        
        assert!(!PlatformUtils::can_delete(&file_path));
    }

    #[test]
    fn test_safe_delete_file() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();
        
        assert!(file_path.exists());
        PlatformUtils::safe_delete(&file_path, false)?;
        assert!(!file_path.exists());
        
        Ok(())
    }

    #[test]
    fn test_safe_delete_directory() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path).unwrap();
        
        assert!(dir_path.exists());
        PlatformUtils::safe_delete(&dir_path, true)?;
        assert!(!dir_path.exists());
        
        Ok(())
    }

    #[test]
    fn test_friendly_error_messages() {
        let perm_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        assert!(PlatformUtils::friendly_error_message(&perm_error).contains("Permission denied"));
        
        let not_found_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        assert!(PlatformUtils::friendly_error_message(&not_found_error).contains("not found"));
    }
}