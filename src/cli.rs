use crate::algorithm::Algorithm;
use crate::hasher;
use crate::output::CsvWriter;
use crate::verifier;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hashcheck")]
#[command(
    version,
    about = "Checksum toolkit CLI - compute, verify, and manage file hashes"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short = 'f', long, value_enum, default_value = "text", global = true)]
    format: OutputFormat,
}

#[derive(Subcommand)]
enum Command {
    Hash {
        file: PathBuf,
        #[arg(short, long, value_enum, default_value = "sha256")]
        algo: Algorithm,
    },
    Dir {
        dir: PathBuf,
        #[arg(short, long, value_enum, default_value = "sha256")]
        algo: Algorithm,
        #[arg(short, long, default_value_t = false)]
        recursive: bool,
        #[arg(long, default_value_t = false)]
        dotfiles: bool,
    },
    Verify {
        checksum_file: PathBuf,
        #[arg(long)]
        base: Option<PathBuf>,
    },
    Genfile {
        dir: PathBuf,
        #[arg(short, long, value_enum, default_value = "sha256")]
        algo: Algorithm,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, default_value_t = false)]
        recursive: bool,
        #[arg(long, default_value_t = false)]
        dotfiles: bool,
    },
    Compare {
        file1: PathBuf,
        file2: PathBuf,
        #[arg(short, long, value_enum, default_value = "sha256")]
        algo: Algorithm,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Hash { file, algo } => cmd_hash(&file, algo, cli.format)?,
        Command::Dir {
            dir,
            algo,
            recursive,
            dotfiles,
        } => cmd_dir(&dir, algo, recursive, dotfiles, cli.format)?,
        Command::Verify {
            checksum_file,
            base,
        } => cmd_verify(&checksum_file, base)?,
        Command::Genfile {
            dir,
            algo,
            output,
            recursive,
            dotfiles,
        } => cmd_genfile(&dir, algo, output, recursive, dotfiles)?,
        Command::Compare { file1, file2, algo } => cmd_compare(&file1, &file2, algo)?,
    }

    Ok(())
}

fn cmd_hash(file: &PathBuf, algo: Algorithm, format: OutputFormat) -> anyhow::Result<()> {
    if file.as_os_str() == "-" {
        let hash = hasher::hash_stdin(algo)?;
        print_single_output(&algo, None, &hash, format);
        return Ok(());
    }
    let hash = hasher::hash_file(file, algo)?;
    print_single_output(&algo, Some(file.to_path_buf()), &hash, format);
    Ok(())
}

fn cmd_dir(
    dir: &PathBuf,
    algo: Algorithm,
    recursive: bool,
    dotfiles: bool,
    format: OutputFormat,
) -> anyhow::Result<()> {
    let mut entries: Vec<PathBuf> = Vec::new();

    for e in walkdir::WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden_path(e.path()) || dotfiles)
        .flatten()
    {
        if e.file_type().is_file() {
            entries.push(e.path().to_path_buf());
        }
    }

    if !recursive {
        entries.retain(|p| p.parent().map(|parent| parent == dir).unwrap_or(false));
    }

    let mut results: Vec<(PathBuf, String)> = Vec::new();
    let mut error_count = 0u32;

    for path in &entries {
        match hasher::hash_file(path, algo) {
            Ok(h) => results.push((path.clone(), h)),
            Err(_) => error_count += 1,
        }
    }

    print_dir_output(&results, &algo, format);

    if error_count > 0 {
        eprintln!("Warning: {} file(s) had errors", error_count);
    }

    Ok(())
}

fn cmd_verify(checksum_file: &PathBuf, base: Option<PathBuf>) -> anyhow::Result<()> {
    let (results, errors) = verifier::verify_checksum_file(checksum_file, base)?;
    print_verify_output(&results, &errors);
    if results.is_empty() && errors.is_empty() {
        anyhow::bail!("No files found in checksum file");
    }
    let has_failures = results.iter().any(|(_, _, ok)| !ok);
    if has_failures || !errors.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

fn cmd_genfile(
    dir: &PathBuf,
    algo: Algorithm,
    output: Option<PathBuf>,
    recursive: bool,
    dotfiles: bool,
) -> anyhow::Result<()> {
    let mut entries: Vec<PathBuf> = Vec::new();

    for e in walkdir::WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden_path(e.path()) || dotfiles)
        .flatten()
    {
        if e.file_type().is_file() {
            entries.push(e.path().to_path_buf());
        }
    }

    if !recursive {
        entries.retain(|p| p.parent().map(|parent| parent == dir).unwrap_or(false));
    }

    let mut lines: Vec<String> = Vec::new();
    let mut error_count = 0u32;

    for path in &entries {
        match hasher::hash_file(path, algo) {
            Ok(h) => {
                let display_path = path
                    .strip_prefix(dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .replace('\\', "/");
                lines.push(format!("{}  {}", h, display_path));
            }
            Err(_) => error_count += 1,
        }
    }

    let output_text = lines.join("\n") + "\n";

    if let Some(ref path) = output {
        let mut f = BufWriter::new(fs::File::create(path)?);
        write!(f, "{}", output_text)?;
        f.flush()?;
    } else {
        print!("{}", output_text);
    }

    if error_count > 0 {
        eprintln!("Warning: {} file(s) had errors", error_count);
    }

    Ok(())
}

fn cmd_compare(file1: &PathBuf, file2: &PathBuf, algo: Algorithm) -> anyhow::Result<()> {
    let hash1 = hasher::hash_file(file1, algo)?;
    let hash2 = hasher::hash_file(file2, algo)?;
    let match_result = hash1 == hash2;

    println!("File 1: {}", file1.display());
    println!("Hash 1: {}", hash1);
    println!("File 2: {}", file2.display());
    println!("Hash 2: {}", hash2);

    if match_result {
        println!("Status: {}", "MATCH".green());
    } else {
        println!("Status: {}", "DIFFER".red());
        std::process::exit(1);
    }

    Ok(())
}

fn is_hidden_path(path: &std::path::Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn print_single_output(algo: &Algorithm, file: Option<PathBuf>, hash: &str, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            if let Some(f) = file {
                println!("{}  {}", hash, f.display());
            } else {
                println!("{}", hash);
            }
        }
        OutputFormat::Json => {
            let mut map = serde_json::Map::new();
            map.insert(
                "algorithm".to_string(),
                serde_json::Value::String(algo.to_string()),
            );
            map.insert(
                "hash".to_string(),
                serde_json::Value::String(hash.to_string()),
            );
            if let Some(f) = file {
                map.insert(
                    "file".to_string(),
                    serde_json::Value::String(f.display().to_string()),
                );
            }
            let output = serde_json::Value::Object(map);
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        OutputFormat::Csv => {
            let mut writer = CsvWriter::new();
            writer.write_record(vec!["algorithm", "hash", "file"]);
            if let Some(f) = &file {
                writer.write_record(vec![&algo.to_string(), hash, &f.display().to_string()]);
            } else {
                writer.write_record(vec![&algo.to_string(), hash, "-"]);
            }
            writer.print();
        }
    }
}

fn print_dir_output(results: &[(PathBuf, String)], algo: &Algorithm, format: OutputFormat) {
    match format {
        OutputFormat::Text => {
            for (file, hash) in results {
                println!("{}  {}", hash, file.display());
            }
        }
        OutputFormat::Json => {
            let mut map = serde_json::Map::new();
            let mut hashes: Vec<serde_json::Value> = Vec::new();
            for (file, hash) in results {
                let mut h = serde_json::Map::new();
                h.insert(
                    "algorithm".to_string(),
                    serde_json::Value::String(algo.to_string()),
                );
                h.insert("hash".to_string(), serde_json::Value::String(hash.clone()));
                h.insert(
                    "file".to_string(),
                    serde_json::Value::String(file.display().to_string()),
                );
                hashes.push(serde_json::Value::Object(h));
            }
            map.insert("files".to_string(), serde_json::Value::Array(hashes));
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap()
            );
        }
        OutputFormat::Csv => {
            let mut writer = CsvWriter::new();
            writer.write_record(vec!["algorithm", "hash", "file"]);
            for (file, hash) in results {
                writer.write_record(vec![&algo.to_string(), hash, &file.display().to_string()]);
            }
            writer.print();
        }
    }
}

fn print_verify_output(results: &[(PathBuf, String, bool)], errors: &[(PathBuf, String)]) {
    let mut all_pass = true;
    for (file, hash, ok) in results {
        if *ok {
            println!("{}: {}", file.display(), "OK".green());
        } else {
            all_pass = false;
            println!(
                "{}: {} (expected {}, got modified)",
                file.display(),
                "FAILED".red(),
                hash
            );
        }
    }
    for (file, err) in errors {
        println!("{}: {} - {}", file.display(), "MISSING".yellow(), err);
    }
    if results.is_empty() && errors.is_empty() {
        eprintln!("No files found in checksum file");
        std::process::exit(1);
    }
    if !all_pass {
        std::process::exit(1);
    }
}
