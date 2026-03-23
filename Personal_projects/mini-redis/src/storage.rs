use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

pub fn create_file(file_name: String) -> Result<(), Box<dyn std::error::Error>> {
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
