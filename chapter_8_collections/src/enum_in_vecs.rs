fn enum_vecs() {
    enum spreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        spreadsheetCell::Int(3),
        spreadsheetCell::Float(1.4),
        spreadsheetCell::Text(String::from("Hello world")),
    ];

    match &row[1] {
        spreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer"),
    }
}
