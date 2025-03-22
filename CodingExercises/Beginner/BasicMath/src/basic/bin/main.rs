fn main(){
    let num1:f32=10.0;
    let num2:f32=12.0;
    println!("{} + {} = {}",num1,num2,add(num1,num2));
    println!("{} - {} = {}",num1,num2,subtract(num1,num2));
    println!("{} * {} = {}",num1,num2,multiply(num1,num2));
    println!("{} / {} = {}",num1,num2,divide(num1,num2));//Since we are passing integer values to the function, if d>n then we will get 0
}

fn add(n1:f32,n2:f32)->f32{
    return n1+n2;
}

fn subtract(n1:f32,n2:f32)->f32{
    return n1-n2;
}

fn multiply(n1:f32,n2:f32)->f32{
    return n1*n2;
}

fn divide(n1:f32,n2:f32)->f32{
    return n1/n2;
}

