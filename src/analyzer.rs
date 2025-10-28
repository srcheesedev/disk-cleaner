use anyhow::Result;
use humansize::{DECIMAL, format_size};
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use tokio::task;

/// Represents a directory or file entry with size information
#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub size_human: String,
    pub is_directory: bool,
}

impl DirectoryEntry {
    pub fn new(path: PathBuf, size_bytes: u64, is_directory: bool) -> Self {
        let size_human = format_size(size_bytes, DECIMAL);
        Self {
            path,
            size_bytes,
            size_human,
            is_directory,
        }
    }
}

/// Handles directory size analysis with async support
pub struct DiskAnalyzer {
    max_depth: usize,
}

impl DiskAnalyzer {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }

    /// Calculate size of a single file or directory
    pub fn calculate_size<P: AsRef<Path>>(path: P) -> Result<u64> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(0);
        }

        if path.is_file() {
            return Ok(path.metadata()?.len());
        }

        let mut total_size = 0u64;
        
        for entry in WalkDir::new(path).follow_links(false) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        if let Ok(metadata) = entry.metadata() {
                            total_size = total_size.saturating_add(metadata.len());
                        }
                    }
                }
                Err(_) => continue, // Skip inaccessible files
            }
        }

        Ok(total_size)
    }

    /// Analyze directory contents and return sorted entries by size
    pub async fn analyze_directory<P: AsRef<Path>>(&self, target_path: P) -> Result<Vec<DirectoryEntry>> {
        let path = target_path.as_ref();
        
        if !path.exists() {
            return Err(anyhow::anyhow!("Directory '{}' does not exist", path.display()));
        }

        if !path.is_dir() {
            return Err(anyhow::anyhow!("'{}' is not a directory", path.display()));
        }

        let mut entries = Vec::new();
        let mut tasks = Vec::new();

        // Read directory entries
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let is_directory = entry_path.is_dir();

            // Spawn async task for size calculation
            let path_clone = entry_path.clone();
            let handle = task::spawn_blocking(move || {
                let size = Self::calculate_size(&path_clone).unwrap_or(0);
                DirectoryEntry::new(path_clone, size, is_directory)
            });

            tasks.push(handle);
        }

        // Collect results
        for task in tasks {
            match task.await {
                Ok(entry) => entries.push(entry),
                Err(e) => eprintln!("Warning: Failed to process entry: {}", e),
            }
        }

        // Sort by size (largest first)
        entries.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

        Ok(entries)
    }

    /// Get entries that match a specific pattern or filter
    pub fn filter_entries(&self, entries: &[DirectoryEntry], min_size: Option<u64>) -> Vec<DirectoryEntry> {
        entries
            .iter()
            .filter(|entry| {
                if let Some(min) = min_size {
                    entry.size_bytes >= min
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;
    use tokio::test;

    fn create_test_structure() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Create files with known sizes
        let mut file1 = File::create(base_path.join("large_file.txt"))?;
        file1.write_all(&vec![b'x'; 1000])?; // 1KB

        let mut file2 = File::create(base_path.join("small_file.txt"))?;
        file2.write_all(&vec![b'y'; 100])?; // 100 bytes

        // Create subdirectory with file
        fs::create_dir(base_path.join("subdir"))?;
        let mut file3 = File::create(base_path.join("subdir/nested_file.txt"))?;
        file3.write_all(&vec![b'z'; 500])?; // 500 bytes

        // Create empty directory
        fs::create_dir(base_path.join("empty_dir"))?;

        Ok(temp_dir)
    }

    #[test]
    async fn test_calculate_file_size() {
        let temp_dir = create_test_structure().unwrap();
        let file_path = temp_dir.path().join("large_file.txt");
        
        let size = DiskAnalyzer::calculate_size(&file_path).unwrap();
        assert_eq!(size, 1000);
    }

    #[test]
    async fn test_calculate_directory_size() {
        let temp_dir = create_test_structure().unwrap();
        let subdir_path = temp_dir.path().join("subdir");
        
        let size = DiskAnalyzer::calculate_size(&subdir_path).unwrap();
        assert_eq!(size, 500);
    }

    #[test]
    async fn test_analyze_directory() {
        let temp_dir = create_test_structure().unwrap();
        let analyzer = DiskAnalyzer::new(1);
        
        let entries = analyzer.analyze_directory(temp_dir.path()).await.unwrap();
        
        // Should have 4 entries: 2 files + 2 directories
        assert_eq!(entries.len(), 4);
        
        // Check if sorted by size (largest first)
        assert!(entries[0].size_bytes >= entries[1].size_bytes);
        
        // Verify specific entries exist
        let names: Vec<String> = entries.iter()
            .map(|e| e.path.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        
        assert!(names.contains(&"large_file.txt".to_string()));
        assert!(names.contains(&"small_file.txt".to_string()));
        assert!(names.contains(&"subdir".to_string()));
        assert!(names.contains(&"empty_dir".to_string()));
    }

    #[test]
    async fn test_directory_entry_creation() {
        let path = PathBuf::from("/test/path");
        let entry = DirectoryEntry::new(path.clone(), 1024, true);
        
        assert_eq!(entry.path, path);
        assert_eq!(entry.size_bytes, 1024);
        assert_eq!(entry.is_directory, true);
        assert_eq!(entry.size_human, "1.02 kB");
    }

    #[test]
    async fn test_filter_entries() {
        let entries = vec![
            DirectoryEntry::new(PathBuf::from("large"), 1000, false),
            DirectoryEntry::new(PathBuf::from("medium"), 500, false),
            DirectoryEntry::new(PathBuf::from("small"), 100, false),
        ];
        
        let analyzer = DiskAnalyzer::new(1);
        let filtered = analyzer.filter_entries(&entries, Some(400));
        
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].path, PathBuf::from("large"));
        assert_eq!(filtered[1].path, PathBuf::from("medium"));
    }

    #[test]
    async fn test_nonexistent_directory() {
        let analyzer = DiskAnalyzer::new(1);
        let result = analyzer.analyze_directory("/nonexistent/path").await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    async fn test_file_instead_of_directory() {
        let temp_dir = create_test_structure().unwrap();
        let file_path = temp_dir.path().join("large_file.txt");
        
        let analyzer = DiskAnalyzer::new(1);
        let result = analyzer.analyze_directory(&file_path).await;
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is not a directory"));
    }

    #[test]
    async fn test_calculate_size_nonexistent() {
        let result = DiskAnalyzer::calculate_size("/nonexistent/file");
        assert_eq!(result.unwrap(), 0);
    }
}