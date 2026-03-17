use colored::Colorize;
use std::{
    env,
    fs::{self, DirEntry, File},
    io::{self, BufRead, BufReader},
    path::Path,
    sync::mpsc,
    thread,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let pattern = match args.get(1) {
        Some(pattern) => {
            let pattern = pattern.to_lowercase();
            pattern
        }
        None => {
            println!("{}", "Missing search pattern".red());
            return;
        }
    };
    let file_path = match args.get(2) {
        Some(path) => path,
        None => {
            println!("{}", "Missing file path".red());
            return;
        }
    };

    if pattern.trim().is_empty() {
        println!("{}", "Please enter input".red());
        return;
    }
    let file_path = Path::new(file_path);

    if !file_path.exists() {
        println!("{}", "Invalid path / File does not exist".red());
        return;
    }
    println!(
        "{:<40} {:>8}  {}",
        "File Path".yellow(),
        "Line No.".yellow(),
        "Text".yellow()
    );
    if file_path.is_file() {
        search_file(file_path, &pattern);
    } else if file_path.is_dir() {
        match search_directory(file_path, &pattern) {
            Ok(_) => {}
            Err(_) => {
                println!("{}", "Failed to enter directory".red());
            }
        };
    }
}

fn search_directory(file_path: &Path, pattern: &str) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        for entry in match fs::read_dir(file_path) {
            Ok(file_path) => file_path,
            Err(_) => {
                println!("Failed to read the file");
                return;
            }
        } {
            let tx = tx.clone();
            let entry: DirEntry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    println!("{}", "Failed to access directory".red());
                    continue;
                }
            };
            let path = entry.path();
            if path.is_file() {
                let value = tx.clone();
                s.spawn(move || {
                    value.send(search_file(&path, &pattern)).unwrap();
                });
                // println!("{:?} - inside search directory", path.display());
            } else if path.is_dir() {
                match search_directory(&path, pattern) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
        }
        drop(tx);
    });
    let results: Vec<_> = rx.iter().collect();
    for result in results {
        // println!("{:?}", result);
        for (path, line, text) in result {
            println!(
                "{:<40} {:>8}  {}",
                path,
                line.to_string().red(),
                text.green()
            );
        }
    }
    Ok(())
}

fn search_file(file_path: &Path, pattern: &str) -> Vec<(String, usize, String)> {
    // println!("{:?} - inside search file", file_path.display());
    let mut match_data = Vec::new();
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", "Failed to open the file".red());
            return match_data;
        }
    };

    let reader = BufReader::new(file);

    for (line_no, line_result) in reader.lines().enumerate() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => {
                println!("{}", "Failed to read the line".red());
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        let lower_line = line.to_lowercase();

        if lower_line.contains(pattern) {
            match_data.push((file_path.display().to_string(), line_no + 1, line));
        }
    }
    match_data
}
