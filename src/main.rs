//! # Disk Cleaner - Interactive Directory Analysis Tool
//!
//! A high-performance, cross-platform disk space analyzer with interactive cleanup capabilities.
//! Built with Rust for maximum performance and safety.
//!
//! Copyright (c) 2025 @srcheesedev
//! Licensed under the MIT License - see LICENSE file for details
//!
//! ## Features
//!
//! - 🚀 **High Performance**: Async directory scanning for large filesystems
//! - 🎯 **Interactive Selection**: Multi-select interface powered by `fzf`-like functionality  
//! - 🔒 **Safe Operations**: Permission checking and validation before deletion
//! - 🌍 **Cross-Platform**: Full Windows, Linux, and macOS support
//! - 📊 **Smart Filtering**: Filter by size, type, and depth
//! - 💾 **Human-Readable**: Beautiful size formatting (KB, MB, GB)
//!
//! ## Usage
//!
//! ```bash
//! # Analyze current directory
//! disk-cleaner
//!
//! # Analyze specific directory with depth limit
//! disk-cleaner /path/to/analyze --depth 3
//!
//! # Show only large files (>100MB)
//! disk-cleaner --min-size 104857600
//!
//! # Show only directories
//! disk-cleaner --dirs-only
//! ```

mod analyzer;
mod file_manager;
mod platform;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use analyzer::DiskAnalyzer;
use file_manager::FileManager;

/// Command-line interface configuration for the disk cleaner application.
///
/// This struct defines all the available command-line options and their
/// default values, using the `clap` crate for automatic parsing and help generation.
#[derive(Parser)]
#[command(name = "disk-cleaner")]
#[command(about = "Interactive directory size analyzer and cleanup tool")]
#[command(version = "0.1.0")]
#[command(long_about = "
🔍 Disk Cleaner - Interactive Directory Analysis Tool

A high-performance, cross-platform disk space analyzer with interactive cleanup capabilities.
Analyze directory sizes, find space hogs, and safely delete unwanted files with an intuitive
multi-select interface.

Features:
  • High-performance async directory scanning
  • Interactive multi-select deletion interface  
  • Cross-platform permission checking
  • Smart filtering by size, type, and depth
  • Beautiful human-readable size formatting
  • Safe operations with validation checks
")]
struct Cli {
    /// Directory to analyze for disk usage
    ///
    /// Specify the target directory to scan. If not provided, analyzes the current directory.
    /// The tool will recursively scan subdirectories up to the specified depth limit.
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum directory depth to analyze
    ///
    /// Controls how deep the recursive directory scan goes. Depth 1 means only immediate
    /// children, depth 2 includes grandchildren, etc. Higher values provide more detail
    /// but take longer to process.
    #[arg(short, long, default_value = "1")]
    depth: usize,

    /// Minimum file/directory size threshold (in bytes)
    ///
    /// Only show entries larger than this size. Useful for finding space hogs.
    /// Examples: 1048576 (1MB), 104857600 (100MB), 1073741824 (1GB)
    /// You can use the constants: --min-size 1048576 for 1MB files and above
    #[arg(short, long)]
    min_size: Option<u64>,

    /// Show only directories in results
    ///
    /// Filter results to display directories only, hiding individual files.
    /// Cannot be used together with --files-only.
    #[arg(long, group = "filter_type")]
    dirs_only: bool,

    /// Show only files in results
    ///
    /// Filter results to display files only, hiding directories.
    /// Cannot be used together with --dirs-only.
    #[arg(long, group = "filter_type")]
    files_only: bool,
}

/// Application entry point.
///
/// Orchestrates the disk analysis workflow by:
/// 1. Parsing command-line arguments
/// 2. Initializing analysis and file management components  
/// 3. Performing directory analysis with filtering
/// 4. Presenting interactive selection interface
/// 5. Safely executing user-confirmed deletions
/// 6. Reporting results and freed space
///
/// # Returns
///
/// `Ok(())` on successful completion, or an error if any step fails.
///
/// # Errors
///
/// Returns error if:
/// - Target directory doesn't exist or isn't accessible
/// - File system operations fail due to permissions
/// - User interface interactions fail
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize components
    let analyzer = DiskAnalyzer::new(cli.depth);
    let file_manager = FileManager::new();

    // Display header
    println!("🔍 Disk Cleaner - Interactive Directory Analysis");
    println!("📁 Analyzing: {}", cli.path.display());

    if cli.depth > 1 {
        println!("📊 Max depth: {}", cli.depth);
    }

    // Analyze directory
    let mut entries = analyzer.analyze_directory(&cli.path).await?;

    // Apply filters
    if let Some(min_size) = cli.min_size {
        entries = analyzer.filter_entries(&entries, Some(min_size));
    }

    if cli.dirs_only {
        entries.retain(|e| e.is_directory);
    } else if cli.files_only {
        entries.retain(|e| !e.is_directory);
    }

    if entries.is_empty() {
        println!("🤷 No entries found matching the criteria.");
        return Ok(());
    }

    // Display summary
    file_manager.display_summary(&entries);

    // Interactive selection
    println!("\n🎯 Select items for deletion:");
    let selected = file_manager.select_entries(&entries)?;

    if selected.is_empty() {
        println!("👋 No items selected. Exiting.");
        return Ok(());
    }

    // Validate entries still exist and check permissions
    let valid_selected = file_manager.validate_entries(&selected);
    let unwritable = file_manager.get_unwritable_entries(&selected);

    if !unwritable.is_empty() {
        println!("\n⚠️  Warning: The following items cannot be deleted (permission denied):");
        for entry in &unwritable {
            println!(
                "  {} {}",
                if entry.is_directory { "📁" } else { "📄" },
                entry.path.display()
            );
        }
        println!("  You may need administrator/root privileges to delete these items.\n");
    }

    if valid_selected.len() != selected.len() {
        let missing = selected.len() - valid_selected.len() - unwritable.len();
        if missing > 0 {
            println!("⚠️  {} selected items no longer exist.", missing);
        }
        if !unwritable.is_empty() {
            println!(
                "⚠️  {} selected items cannot be deleted due to permissions.",
                unwritable.len()
            );
        }
        println!("📊 Proceeding with {} valid items.", valid_selected.len());
    }

    if valid_selected.is_empty() {
        println!("❌ No valid items to delete.");
        return Ok(());
    }

    // Confirm deletion
    if file_manager.confirm_deletion(&valid_selected)? {
        println!("\n🗑️  Proceeding with deletion...");

        let (deleted, failed) = file_manager.delete_entries(&valid_selected)?;

        // Display results
        if !deleted.is_empty() {
            println!("\n✅ Successfully deleted {} items:", deleted.len());
            for item in &deleted {
                println!("  🗑️  {}", item);
            }
        }

        if !failed.is_empty() {
            println!("\n❌ Failed to delete {} items:", failed.len());
            for item in &failed {
                println!("  ⚠️  {}", item);
            }
        }

        // Calculate freed space
        let freed_bytes: u64 = valid_selected
            .iter()
            .filter(|entry| {
                deleted
                    .iter()
                    .any(|d| d.contains(&entry.path.to_string_lossy().to_string()))
            })
            .map(|entry| entry.size_bytes)
            .sum();

        if freed_bytes > 0 {
            let freed_human = humansize::format_size(freed_bytes, humansize::DECIMAL);
            println!("\n💾 Total space freed: {}", freed_human);
        }

        println!("\n🏁 Operation completed!");
    } else {
        println!("❌ Deletion cancelled by user.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test default values
        let cli = Cli::parse_from(["disk-cleaner"]);
        assert_eq!(cli.path, PathBuf::from("."));
        assert_eq!(cli.depth, 1);
        assert_eq!(cli.min_size, None);
        assert!(!cli.dirs_only);
        assert!(!cli.files_only);
    }

    #[test]
    fn test_cli_with_arguments() {
        let cli = Cli::parse_from([
            "disk-cleaner",
            "/tmp",
            "--depth",
            "2",
            "--min-size",
            "1000",
            "--dirs-only",
        ]);

        assert_eq!(cli.path, PathBuf::from("/tmp"));
        assert_eq!(cli.depth, 2);
        assert_eq!(cli.min_size, Some(1000));
        assert!(cli.dirs_only);
        assert!(!cli.files_only);
    }

    #[test]
    fn test_cli_conflicting_flags_prevented() {
        use clap::error::ErrorKind;

        // This should fail now with the group constraint
        let result = Cli::try_parse_from(["disk-cleaner", "--dirs-only", "--files-only"]);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
        }
    }
}
