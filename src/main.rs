mod analyzer;
mod file_manager;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use analyzer::DiskAnalyzer;
use file_manager::FileManager;

#[derive(Parser)]
#[command(name = "disk-cleaner")]
#[command(about = "Interactive directory size analyzer and cleanup tool")]
#[command(version = "0.1.0")]
struct Cli {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum depth to analyze
    #[arg(short, long, default_value = "1")]
    depth: usize,

    /// Minimum size to display (in bytes)
    #[arg(short, long)]
    min_size: Option<u64>,

    /// Show only directories
    #[arg(long)]
    dirs_only: bool,

    /// Show only files
    #[arg(long)]
    files_only: bool,
}

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

    // Validate entries still exist
    let valid_selected = file_manager.validate_entries(&selected);
    if valid_selected.len() != selected.len() {
        println!("⚠️  Some selected items no longer exist. Proceeding with valid items only.");
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
        let freed_bytes: u64 = valid_selected.iter()
            .filter(|entry| deleted.iter().any(|d| d.contains(&entry.path.to_string_lossy().to_string())))
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
        let cli = Cli::parse_from(&["disk-cleaner"]);
        assert_eq!(cli.path, PathBuf::from("."));
        assert_eq!(cli.depth, 1);
        assert_eq!(cli.min_size, None);
        assert!(!cli.dirs_only);
        assert!(!cli.files_only);
    }

    #[test]
    fn test_cli_with_arguments() {
        let cli = Cli::parse_from(&[
            "disk-cleaner", 
            "/tmp", 
            "--depth", "2", 
            "--min-size", "1000",
            "--dirs-only"
        ]);
        
        assert_eq!(cli.path, PathBuf::from("/tmp"));
        assert_eq!(cli.depth, 2);
        assert_eq!(cli.min_size, Some(1000));
        assert!(cli.dirs_only);
        assert!(!cli.files_only);
    }

    #[test]
    fn test_cli_conflicting_flags() {
        // This should work - last flag wins typically
        let cli = Cli::parse_from(&[
            "disk-cleaner",
            "--dirs-only",
            "--files-only"
        ]);
        
        // Both flags can be set, but logic in main() should handle conflicts
        assert!(cli.dirs_only);
        assert!(cli.files_only);
    }
}