use std::io::{self,  Write};

fn main() {
    print!("Enter a number(integer): ");
    let _=io::stdout().flush();

    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Please provide valid input");

    let num:i32=input.trim().parse().expect("Unable to parse provided input to integer. Please provide a valid integer as input");

    if num%2==1 {
        println!("{} is odd",num);
    }
    else {
        println!("{} is even",num)
    }
    //let _=io::stdout().flush();
}
