use std::io::{stdin,stdout, Write};

fn main(){
    let mut input1=String::new();
    let mut input2=String::new();
    let stdin=stdin();
    print!("Enter first number: ");
    let _= stdout().flush();
    stdin.read_line(&mut input1).expect("Expected proper input");
    print!("Enter second number: ");
    let _= stdout().flush();
    stdin.read_line(&mut input2).expect("Expected proper input");

    let num1:f64=input1.trim().parse().expect("Failed to parse gven input. Please give a number as input");
    let num2:f64=input2.trim().parse().expect("Failed to parse gven input. Please give a number as input");

    println!("{} + {} = {}",num1,num2,num1+num2);
    println!("{} - {} = {}",num1,num2,num1-num2);
    println!("{} * {} = {}",num1,num2,num1*num2);
    println!("{} / {} = {}",num1,num2,num1/num2);
}