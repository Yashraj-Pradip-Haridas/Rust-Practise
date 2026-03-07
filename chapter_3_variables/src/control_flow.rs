fn cont_flow(){
    let number = 5;

    // In rust the condition must be explicitly a boolean
    if number< 10{
        println!("Number less than 10");
    }
    else if number >10{
        println!("Number greater than 10");
    }
    else {
        println!("Number equal to 10");
    }

    let condition = true;
    let number  = if condition {20} else {21};

}