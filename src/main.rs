use clap::Parser;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

/// TidyUp: A CLI to organize your messy folders.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the directory you want to clean up
    #[arg(short, long, default_value = ".")]
    path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let target_dir = Path::new(&cli.path);

    if !target_dir.exists() {
        anyhow::bail!("Directory does not exist: {}", cli.path);
    }

    println!("🧹 Tidying up directory: {:?}", target_dir);

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();
                let category = match ext_str.as_str() {
                    "jpg" | "png" | "gif" | "svg" => "Images",
                    "pdf" | "doc" | "docx" | "txt" | "md" => "Documents",
                    "zip" | "tar" | "gz" | "rar" => "Archives",
                    "exe" | "deb" | "msi" | "dmg" => "Installers",
                    "rs" | "py" | "js" | "html" | "css" => "Code",
                    _ => "Misc",
                };

                let category_path = target_dir.join(category);
                fs::create_dir_all(&category_path)?;

                let file_name = path.file_name().unwrap();
                let new_path = category_path.join(file_name);

                fs::rename(&path, &new_path)
                    .with_context(|| format!("Failed to move {:?}", path))?;
                
                println!("Moved {:?} -> {}/", file_name, category);
            }
        }
    }

    println!("✨ All done! Directory is clean.");
    Ok(())
}