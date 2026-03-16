use colored::Colorize;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
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
    search_file(file_path, pattern);
}




fn search_file(file_path: &Path, pattern: String) {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", "Failed to open the file".red());
            return;
        }
    };

    let reader = BufReader::new(file);

    println!("{:>8}  {}", "Line No.".yellow(), "Text".yellow());
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

        if lower_line.contains(&pattern) {
            println!("{:>8}  {}", line_no.to_string().red(), line.green());
        }
    }
}
