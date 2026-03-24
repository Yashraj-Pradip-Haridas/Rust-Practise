use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Seek, Write},
    path::Path,
};

use serde::de::value::Error;

use crate::SHARED_DATA;

pub fn create_file(file_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&file_name);
    if path.exists() {
        return Ok(());
    }
    let _file = match File::create(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to create file");
            return Err("Failed to create the file".into());
        }
    };
    Ok(())
}

pub fn write_to_file(file_name: String, data: String) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    let mut writer = BufWriter::new(&file);
    writeln!(writer, "{}", data)?;
    Ok(())
}

pub fn clear_log_file(file_name: String) -> std::io::Result<()> {
    let _file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;
    Ok(())
}
pub fn create_snapshot(file_name: String) -> Result<(), Box<dyn std::error::Error>> {
    {
        create_file(file_name.to_string())?;
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)?;
        let data = SHARED_DATA.lock().unwrap();
        let writer = BufWriter::new(file);
        match serde_json::to_writer_pretty(writer, &*data) {
            Ok(_) => {
                println!("Snapshot created successfully");
            }
            Err(_) => {
                return Err("Failed to create snapshot".into());
            }
        };
    }
    return Ok(());
}
