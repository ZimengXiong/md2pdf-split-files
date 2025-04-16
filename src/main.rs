use clap::Parser;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about = "Convert Markdown to PDF with support for \\newfile")]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long, default_value = "output")]
    output_dir: PathBuf,

    #[arg(last = true)]
    pandoc_args: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    fs::create_dir_all(&args.output_dir)?;

    let file = File::open(&args.input)?;
    let reader = BufReader::new(file);

    let mut current_content = String::new();
    let mut current_filename = String::new();
    let mut prev_was_newfile = false;

    for line in reader.lines() {
        let line = line?;

        let trimmed = line.trim();

        // If both \newpage and \newfile appear on the same line, prefer \newfile and ignore \newpage
        if trimmed.contains("\\newfile") {
            prev_was_newfile = true;
            if !current_content.is_empty() && !current_filename.is_empty() {
                process_markdown_to_pdf(
                    &current_content,
                    &current_filename,
                    &args.output_dir,
                    &args.pandoc_args,
                )?;
            }
            current_content.clear();
            current_filename.clear();
            continue;
        }

        if trimmed == "\\newpage" && prev_was_newfile {
            continue;
        }

        if trimmed == "\\newpage" {
            prev_was_newfile = false;
            continue;
        }

        prev_was_newfile = false;

        if current_filename.is_empty() && (line.starts_with("# ") || line.starts_with("## ")) {
            current_filename = line
                .trim_start_matches('#')
                .trim()
                .to_lowercase()
                .replace(' ', "_")
                + ".pdf";
        }

        current_content.push_str(&line);
        current_content.push('\n');
    }

    if !current_content.is_empty() && !current_filename.is_empty() {
        process_markdown_to_pdf(
            &current_content,
            &current_filename,
            &args.output_dir,
            &args.pandoc_args,
        )?;
    }

    Ok(())
}

fn process_markdown_to_pdf(
    content: &str,
    filename: &str,
    output_dir: &Path,
    pandoc_args: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    use tempfile::Builder;

    // Use a .md extension for the temp file to help pandoc deduce the format
    let mut temp_file = Builder::new()
        .suffix(".md")
        .tempfile_in(output_dir)?;
    temp_file.write_all(content.as_bytes())?;
    let temp_md_path = temp_file.path();

    let output_pdf = output_dir.join(filename);
    let mut cmd = Command::new("pandoc");
    cmd.arg(&temp_md_path).arg("-o").arg(&output_pdf).arg("-s");
    for arg in pandoc_args {
        cmd.arg(arg);
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(format!("pandoc failed for {}", filename).into());
    }

    println!("Generated PDF: {}", filename);
    Ok(())
}
