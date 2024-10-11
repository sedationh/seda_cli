use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Code {
        #[clap(value_parser)]
        repo_url: String,
        #[clap(value_parser)]
        new_name: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Code { repo_url, new_name } => {
            let repo_name = repo_url.split('/').last().unwrap().trim_end_matches(".git");
            let new_name = new_name.as_deref().unwrap_or(repo_name);

            let current_dir = std::env::current_dir()?;
            let target_dir = current_dir.join(new_name);

            if target_dir.exists() {
                println!("Opening existing directory: {}", target_dir.display());
                open_vscode(&target_dir)?;
            } else {
                println!("Cloning repository: {}", repo_url);
                match clone_repo(repo_url, &target_dir) {
                    Ok(_) => {
                        println!("Repository cloned successfully.");
                        println!("Opening new directory: {}", target_dir.display());
                        open_vscode(&target_dir)?;
                    }
                    Err(e) => {
                        eprintln!("Failed to clone repository: {}", e);
                        eprintln!("Not opening VSCode due to clone failure.");
                    }
                }
            }
        }
    }

    Ok(())
}

fn clone_repo(url: &str, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(target)
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(error_message.to_string().into());
    }

    Ok(())
}

fn open_vscode(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let editor = env::var("VSCODE_ALTERNATIVE").unwrap_or_else(|_| "code".to_string());
    Command::new(editor).arg(path).status()?;
    Ok(())
}