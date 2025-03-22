fn main() {
    let mut i_n=0;
    let mut i_n1=1;
    let mut n=2;
    println!("{}",i_n);
    println!("{}",i_n1);
    while n<10 {
        let i_now=i_n+i_n1;
        println!("{}",i_now);
        i_n=i_n1;
        i_n1=i_now;
        n+=1;
    }
}
