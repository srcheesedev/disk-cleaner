//! # File Management and User Interface Module
//!
//! Provides safe file operations and interactive user interfaces for the disk cleaner.
//! This module handles all user interactions, file deletion operations, and safety validations.
//!
//! Copyright (c) 2025 @srcheesedev
//! Licensed under the MIT License - see LICENSE file for details
//!
//! ## Key Features
//!
//! - **Interactive Selection**: Multi-select interface with beautiful formatting
//! - **Safety First**: Comprehensive validation and permission checking before operations
//! - **Cross-Platform**: Uses platform-specific utilities for reliable file operations
//! - **User Feedback**: Clear progress indicators and detailed error reporting
//! - **Confirmation Flows**: Multiple confirmation steps to prevent accidental deletions
//!
//! ## Safety Guarantees
//!
//! - **Permission Validation**: Checks write permissions before attempting deletions
//! - **Existence Verification**: Validates all paths exist before operations
//! - **User Confirmation**: Requires explicit user confirmation for all destructive operations
//! - **Atomic Operations**: Each file operation is handled independently with proper error handling
//! - **Detailed Reporting**: Comprehensive success/failure reporting with specific error messages
//!
//! ## Usage Example
//!
//! ```rust
//! let manager = FileManager::new();
//! let selected = manager.interactive_select(&entries)?;
//! let valid = manager.validate_entries(&selected);
//!
//! if manager.confirm_deletion(&valid)? {
//!     let (deleted, failed) = manager.delete_entries(&valid)?;
//! }
//! ```

use crate::analyzer::DirectoryEntry;
use crate::platform::PlatformUtils;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::path::Path;

// Constants for UI formatting
const TABLE_WIDTH: usize = 60;
const SIZE_COLUMN_WIDTH: usize = 8;
const TYPE_COLUMN_WIDTH: usize = 4;

/// Handles user interaction for file selection and deletion
/// Safe file operations manager with interactive user interface capabilities.
///
/// The `FileManager` provides a safe, user-friendly interface for file operations
/// with comprehensive validation, permission checking, and interactive confirmation flows.
/// All operations are designed with safety as the primary concern.
///
/// # Safety Features
///
/// - **Multi-step Validation**: Validates existence, permissions, and user intent
/// - **Cross-platform Support**: Uses platform-specific utilities for reliable operations
/// - **Detailed Reporting**: Comprehensive success/failure reporting with specific errors
/// - **User Confirmation**: Requires explicit confirmation before any destructive operations
/// - **Atomic Operations**: Each file/directory operation is handled independently
///
/// # User Interface
///
/// - **Multi-select Interface**: Beautiful, intuitive selection interface using `dialoguer`
/// - **Formatted Display**: Clear size formatting and file type indicators
/// - **Progress Feedback**: Real-time progress reporting during operations
/// - **Error Communication**: User-friendly error messages with actionable suggestions
///
/// # Examples
///
/// ```rust
/// let manager = FileManager::new();
///
/// // Interactive file selection
/// let selected = manager.interactive_select(&all_entries)?;
///
/// // Validate before operation
/// let valid = manager.validate_entries(&selected);
/// let unwritable = manager.get_unwritable_entries(&selected);
///
/// // Get user confirmation
/// if manager.confirm_deletion(&valid)? {
///     let (deleted, failed) = manager.delete_entries(&valid)?;
///     println!("Successfully deleted {} items", deleted.len());
/// }
/// ```
pub struct FileManager {
    theme: ColorfulTheme,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    /// Display entries in a formatted way and allow multi-selection
    pub fn select_entries(&self, entries: &[DirectoryEntry]) -> Result<Vec<DirectoryEntry>> {
        if entries.is_empty() {
            println!("No entries found to select from.");
            return Ok(vec![]);
        }

        // Create selection items with formatted display
        let items: Vec<String> = entries
            .iter()
            .map(|entry| {
                let file_type = if entry.is_directory { "DIR " } else { "FILE" };
                let name = entry
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_else(|| entry.path.to_string_lossy());
                format!("{:>width_size$} {:>width_type$} {}", 
                    entry.size_human, 
                    file_type, 
                    name,
                    width_size = SIZE_COLUMN_WIDTH,
                    width_type = TYPE_COLUMN_WIDTH
                )
            })
            .collect();

        let selection = MultiSelect::with_theme(&self.theme)
            .with_prompt("Select items to delete (use SPACE to select, ENTER to confirm)")
            .items(&items)
            .interact_opt()?;

        match selection {
            Some(indices) => {
                let selected: Vec<DirectoryEntry> =
                    indices.into_iter().map(|i| entries[i].clone()).collect();
                Ok(selected)
            }
            None => {
                println!("Selection cancelled.");
                Ok(vec![])
            }
        }
    }

    /// Confirm deletion with user
    pub fn confirm_deletion(&self, entries: &[DirectoryEntry]) -> Result<bool> {
        if entries.is_empty() {
            return Ok(false);
        }

        println!("\n🚨 WARNING: The following items will be permanently deleted:");

        let mut total_size = 0u64;
        for entry in entries {
            let file_type = if entry.is_directory { "DIR " } else { "FILE" };
            println!(
                "  {:>width_size$} {:>width_type$} {}",
                entry.size_human,
                file_type,
                entry.path.display(),
                width_size = SIZE_COLUMN_WIDTH,
                width_type = TYPE_COLUMN_WIDTH
            );
            total_size += entry.size_bytes;
        }

        let total_human = humansize::format_size(total_size, humansize::DECIMAL);
        println!("\n💾 Total size to be freed: {}", total_human);

        let confirmed = Confirm::with_theme(&self.theme)
            .with_prompt("Are you absolutely sure you want to delete these items?")
            .default(false)
            .interact()?;

        Ok(confirmed)
    }

    /// Delete selected files and directories
    pub fn delete_entries(&self, entries: &[DirectoryEntry]) -> Result<(Vec<String>, Vec<String>)> {
        let mut deleted = Vec::new();
        let mut failed = Vec::new();

        for (i, entry) in entries.iter().enumerate() {
            print!(
                "Deleting {}/{}: {}... ",
                i + 1,
                entries.len(),
                entry.path.display()
            );

            match self.delete_single_entry(&entry.path, entry.is_directory) {
                Ok(()) => {
                    println!("✅");
                    deleted.push(entry.path.to_string_lossy().to_string());
                }
                Err(e) => {
                    // Use friendly error message for better user experience
                    let friendly_msg = if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                        PlatformUtils::friendly_error_message(io_error)
                    } else {
                        e.to_string()
                    };
                    println!("❌ ({})", friendly_msg);
                    failed.push(format!("{} ({})", entry.path.display(), friendly_msg));
                }
            }
        }

        Ok((deleted, failed))
    }

    /// Delete a single file or directory with cross-platform support
    fn delete_single_entry<P: AsRef<Path>>(&self, path: P, is_directory: bool) -> Result<()> {
        PlatformUtils::safe_delete(path, is_directory)
    }

    /// Validate that all entries still exist and can be deleted before deletion
    pub fn validate_entries(&self, entries: &[DirectoryEntry]) -> Vec<DirectoryEntry> {
        entries
            .iter()
            .filter(|entry| entry.path.exists() && PlatformUtils::can_delete(&entry.path))
            .cloned()
            .collect()
    }

    /// Get entries that exist but cannot be deleted (for warning the user)
    pub fn get_unwritable_entries(&self, entries: &[DirectoryEntry]) -> Vec<DirectoryEntry> {
        entries
            .iter()
            .filter(|entry| entry.path.exists() && !PlatformUtils::can_delete(&entry.path))
            .cloned()
            .collect()
    }

    /// Display summary of files in a table format
    pub fn display_summary(&self, entries: &[DirectoryEntry]) {
        if entries.is_empty() {
            println!("No files or directories found.");
            return;
        }

        println!("\n📊 Directory Contents (sorted by size):");
        println!("{:-<width$}", "", width = TABLE_WIDTH);
        println!("{:>width_size$} {:>width_type$} NAME", 
            "SIZE", 
            "TYPE",
            width_size = SIZE_COLUMN_WIDTH,
            width_type = TYPE_COLUMN_WIDTH
        );
        println!("{:-<width$}", "", width = TABLE_WIDTH);

        for entry in entries {
            let file_type = if entry.is_directory { "DIR " } else { "FILE" };
            let name = entry
                .path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_else(|| entry.path.to_string_lossy());
            println!("{:>width_size$} {:>width_type$} {}", 
                entry.size_human, 
                file_type, 
                name,
                width_size = SIZE_COLUMN_WIDTH,
                width_type = TYPE_COLUMN_WIDTH
            );
        }

        let total_size: u64 = entries.iter().map(|e| e.size_bytes).sum();
        let total_human = humansize::format_size(total_size, humansize::DECIMAL);
        println!("{:-<width$}", "", width = TABLE_WIDTH);
        println!("{:>width_size$} {:>width_type$} TOTAL", 
            total_human, 
            "",
            width_size = SIZE_COLUMN_WIDTH,
            width_type = TYPE_COLUMN_WIDTH
        );
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_files() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path();

        // Create test files
        let mut file1 = File::create(base_path.join("test1.txt"))?;
        file1.write_all(b"test content 1")?;

        let mut file2 = File::create(base_path.join("test2.txt"))?;
        file2.write_all(b"test content 2")?;

        // Create test directory
        fs::create_dir(base_path.join("test_dir"))?;
        let mut file3 = File::create(base_path.join("test_dir/nested.txt"))?;
        file3.write_all(b"nested content")?;

        Ok(temp_dir)
    }

    #[test]
    fn test_file_manager_creation() {
        let _manager = FileManager::new();
        // Should create without panicking - no assertion needed
    }

    #[test]
    fn test_delete_single_file() -> Result<()> {
        let temp_dir = create_test_files()?;
        let file_path = temp_dir.path().join("test1.txt");

        assert!(file_path.exists());

        let manager = FileManager::new();
        manager.delete_single_entry(&file_path, false)?;

        assert!(!file_path.exists());
        Ok(())
    }

    #[test]
    fn test_delete_single_directory() -> Result<()> {
        let temp_dir = create_test_files()?;
        let dir_path = temp_dir.path().join("test_dir");

        assert!(dir_path.exists());
        assert!(dir_path.is_dir());

        let manager = FileManager::new();
        manager.delete_single_entry(&dir_path, true)?;

        assert!(!dir_path.exists());
        Ok(())
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let manager = FileManager::new();
        let result = manager.delete_single_entry("/nonexistent/file.txt", false);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validate_entries() -> Result<()> {
        let temp_dir = create_test_files()?;
        let manager = FileManager::new();

        let entries = vec![
            DirectoryEntry::new(temp_dir.path().join("test1.txt"), 100, false),
            DirectoryEntry::new(temp_dir.path().join("nonexistent.txt"), 200, false),
            DirectoryEntry::new(temp_dir.path().join("test_dir"), 300, true),
        ];

        let valid_entries = manager.validate_entries(&entries);

        // Should only include existing files
        assert_eq!(valid_entries.len(), 2);
        assert!(valid_entries
            .iter()
            .any(|e| e.path.file_name().unwrap() == "test1.txt"));
        assert!(valid_entries
            .iter()
            .any(|e| e.path.file_name().unwrap() == "test_dir"));

        Ok(())
    }

    #[test]
    fn test_delete_entries() -> Result<()> {
        let temp_dir = create_test_files()?;
        let manager = FileManager::new();

        let entries = vec![
            DirectoryEntry::new(temp_dir.path().join("test1.txt"), 100, false),
            DirectoryEntry::new(temp_dir.path().join("test2.txt"), 200, false),
        ];

        let (deleted, failed) = manager.delete_entries(&entries)?;

        assert_eq!(deleted.len(), 2);
        assert_eq!(failed.len(), 0);

        // Verify files are actually deleted
        assert!(!temp_dir.path().join("test1.txt").exists());
        assert!(!temp_dir.path().join("test2.txt").exists());

        Ok(())
    }

    #[test]
    fn test_delete_entries_with_failures() -> Result<()> {
        let temp_dir = create_test_files()?;
        let manager = FileManager::new();

        let entries = vec![
            DirectoryEntry::new(temp_dir.path().join("test1.txt"), 100, false),
            DirectoryEntry::new(temp_dir.path().join("nonexistent.txt"), 200, false),
        ];

        let (deleted, failed) = manager.delete_entries(&entries)?;

        assert_eq!(deleted.len(), 1);
        assert_eq!(failed.len(), 1);

        assert!(deleted[0].contains("test1.txt"));
        assert!(failed[0].contains("nonexistent.txt"));

        Ok(())
    }

    #[test]
    fn test_display_summary_empty() {
        let manager = FileManager::new();
        let entries: Vec<DirectoryEntry> = vec![];

        // Should not panic
        manager.display_summary(&entries);
    }

    #[test]
    fn test_display_summary_with_entries() {
        let manager = FileManager::new();
        let entries = vec![
            DirectoryEntry::new(PathBuf::from("large.txt"), 1000, false),
            DirectoryEntry::new(PathBuf::from("small_dir"), 500, true),
        ];

        // Should not panic
        manager.display_summary(&entries);
    }
}
