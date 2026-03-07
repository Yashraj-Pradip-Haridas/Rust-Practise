fn loops(){

    let mut counter  = 0;
    let result = loop{
        counter+=1; 
        if counter == 10 {
            break counter;
        }
    };

    loop{
        println!("Executes till break is called, this loop can also return values");
        if true{
            break;
        }
    }


    // classic while loop 
    while counter!=0{
        println!("Number still not 0");
        counter-=1;
    }

    // for end loop -  used while looping a collection
    let a = [10,20,30,40,50];

    for element in a.iter(){
        println!("{}", element);
    }

    // Last number is exclusive
    for number in 1..4 {
        println!("{}", number);
    }

    // Two types of comments in rust
    // line comment
    /*
        Block comments
     */
}