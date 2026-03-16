use colored::Colorize;
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::Path,
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
            Ok(_) => println!("{}", "Success".green()),
            Err(_) => println!("{}", "Failed".red()),
        };
    }
}

fn search_directory(file_path: &Path, pattern: &String) -> io::Result<()> {
    for entry in fs::read_dir(file_path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                println!("{}", "Failed to access directory".red());
                continue;
            }
        };
        let path = entry.path();
        if path.is_file() {
            // println!("{:?} - inside search directory", path.display());
            search_file(&path, &pattern);
        } else if path.is_dir() {
            match search_directory(&path, pattern) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
    }
    Ok(())
}

fn search_file(file_path: &Path, pattern: &String) {
    // println!("{:?} - inside search file", file_path.display());
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", "Failed to open the file".red());
            return;
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
            println!(
                "{:<40} {:>8}  {}",
                file_path.display(),
                (line_no + 1).to_string().red(),
                line.green()
            );
        }
    }
}
