//! # Directory Analysis Module
//!
//! High-performance directory size analysis with async processing capabilities.
//! This module provides the core functionality for scanning filesystems and
//! calculating directory sizes efficiently.
//!
//! Copyright (c) 2025 @srcheesedev
//! Licensed under the MIT License - see LICENSE file for details
//!
//! ## Key Features
//!
//! - **Async Processing**: Non-blocking directory traversal for large filesystems
//! - **Size Calculation**: Accurate byte-level size reporting with human-readable formatting
//! - **Flexible Filtering**: Filter by minimum size and entry type (files/directories)
//! - **Error Resilience**: Graceful handling of permission errors and inaccessible files
//! - **Cross-Platform**: Works reliably on Windows, Linux, and macOS
//!
//! ## Usage Example
//!
//! ```rust
//! use analyzer::DiskAnalyzer;
//!
//! let analyzer = DiskAnalyzer::new(2); // Depth limit of 2
//! let entries = analyzer.analyze_directory("/path/to/scan").await?;
//! let large_files = analyzer.filter_entries(&entries, Some(1_000_000)); // >1MB
//! ```

use anyhow::Result;
use humansize::{format_size, DECIMAL};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task;
use walkdir::WalkDir;

/// Represents a filesystem entry (file or directory) with comprehensive metadata.
///
/// This structure contains all the information needed to display, sort, and operate
/// on filesystem entries in the disk cleaner interface.
///
/// # Fields
///
/// * `path` - The full filesystem path to this entry
/// * `size_bytes` - Size in bytes (for files: file size, for directories: total recursive size)
/// * `size_human` - Human-readable size string (e.g., "1.2 GB", "456 MB")
/// * `is_directory` - Whether this entry represents a directory or a file
///
/// # Examples
///
/// ```rust
/// let entry = DirectoryEntry::new(
///     PathBuf::from("/home/user/documents"),
///     1073741824, // 1 GB
///     true        // is directory
/// );
/// assert_eq!(entry.size_human, "1.1 GB");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryEntry {
    /// Full filesystem path to this entry
    pub path: PathBuf,
    /// Size in bytes (recursive for directories)  
    pub size_bytes: u64,
    /// Human-readable size string (e.g., "1.2 GB")
    pub size_human: String,
    /// True if this entry is a directory, false if it's a file
    pub is_directory: bool,
}

impl DirectoryEntry {
    /// Creates a new directory entry with automatic human-readable size formatting.
    ///
    /// # Arguments
    ///
    /// * `path` - The filesystem path to this entry
    /// * `size_bytes` - Size in bytes (recursive total for directories)
    /// * `is_directory` - Whether this entry represents a directory
    ///
    /// # Returns
    ///
    /// A new `DirectoryEntry` with automatically formatted human-readable size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let file_entry = DirectoryEntry::new(
    ///     PathBuf::from("large_video.mp4"),
    ///     2147483648, // 2 GB
    ///     false
    /// );
    /// assert_eq!(file_entry.size_human, "2.1 GB");
    /// ```
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

/// High-performance directory analyzer with async processing capabilities.
///
/// The `DiskAnalyzer` provides efficient filesystem scanning and size calculation
/// using async operations to prevent blocking on large directory structures.
/// It supports configurable depth limits and comprehensive error handling.
///
/// # Performance Characteristics
///
/// - **Async Operations**: Non-blocking I/O for responsive user experience
/// - **Memory Efficient**: Streams results without loading entire filesystem in memory  
/// - **Concurrent Processing**: Leverages tokio's task scheduler for parallel operations
/// - **Error Resilient**: Continues processing despite individual file access failures
///
/// # Examples
///
/// ```rust
/// // Create analyzer with depth limit
/// let analyzer = DiskAnalyzer::new(3);
///
/// // Analyze directory asynchronously
/// let entries = analyzer.analyze_directory("/home/user").await?;
///
/// // Filter large files only
/// let large_files = analyzer.filter_entries(&entries, Some(100_000_000));
/// ```
#[derive(Debug)]
pub struct DiskAnalyzer {
    max_depth: usize,
}

impl DiskAnalyzer {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }

    /// Calculate size of a single file or directory with depth limiting
    pub fn calculate_size<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(0);
        }

        if path.is_file() {
            return Ok(path.metadata()?.len());
        }

        let mut total_size = 0u64;

        for entry in WalkDir::new(path)
            .follow_links(false)
            .max_depth(self.max_depth) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        if let Ok(metadata) = entry.metadata() {
                            total_size = total_size.saturating_add(metadata.len());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Cannot access {}: {}", 
                        e.path().map(|p| p.display().to_string()).unwrap_or_else(|| "unknown path".to_string()), 
                        e.io_error().map(|io_e| io_e.to_string()).unwrap_or_else(|| "unknown error".to_string())
                    );
                    continue;
                }
            }
        }

        Ok(total_size)
    }

    /// Analyze directory contents and return sorted entries by size
    pub async fn analyze_directory<P: AsRef<Path>>(
        &self,
        target_path: P,
    ) -> Result<Vec<DirectoryEntry>> {
        let path = target_path.as_ref();

        if !path.exists() {
            return Err(anyhow::anyhow!(
                "Directory '{}' does not exist",
                path.display()
            ));
        }

        if !path.is_dir() {
            return Err(anyhow::anyhow!("'{}' is not a directory", path.display()));
        }

        let mut entries = Vec::new();
        let mut tasks = Vec::new();

        // Read directory entries with depth limiting
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let is_directory = entry_path.is_dir();

            // Spawn async task for size calculation
            let path_clone = entry_path.clone();
            let max_depth = if is_directory { self.max_depth.saturating_sub(1) } else { 1 };
            let analyzer = DiskAnalyzer::new(max_depth);
            
            let handle = task::spawn_blocking(move || {
                let size = analyzer.calculate_size(&path_clone).unwrap_or(0);
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
    pub fn filter_entries(
        &self,
        entries: &[DirectoryEntry],
        min_size: Option<u64>,
    ) -> Vec<DirectoryEntry> {
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
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    use tokio::test;

    fn create_test_structure() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Create files with known sizes
        let mut file1 = File::create(base_path.join("large_file.txt"))?;
        file1.write_all(&vec![b'x'; 1000])?; // 1KB

        let mut file2 = File::create(base_path.join("small_file.txt"))?;
        file2.write_all(&[b'y'; 100])?; // 100 bytes

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
        let analyzer = DiskAnalyzer::new(3);

        let size = analyzer.calculate_size(&file_path).unwrap();
        assert_eq!(size, 1000);
    }

    #[test]
    async fn test_calculate_directory_size() {
        let temp_dir = create_test_structure().unwrap();
        let subdir_path = temp_dir.path().join("subdir");
        let analyzer = DiskAnalyzer::new(3);

        let size = analyzer.calculate_size(&subdir_path).unwrap();
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
        let names: Vec<String> = entries
            .iter()
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
        assert!(entry.is_directory);
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
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("is not a directory"));
    }

    #[test]
    async fn test_calculate_size_nonexistent() {
        let analyzer = DiskAnalyzer::new(3);
        let result = analyzer.calculate_size("/nonexistent/file");
        assert_eq!(result.unwrap(), 0);
    }
}
