fn main() {
    print_phrase("i start learning rust");
    println!("{}",gcd(150,12));
    println!("{}", multiple_return_values(true));
    println!("{}", multiple_return_values(false));
}
fn print_phrase(p1 :&str){
    println!("Hello from another function, {}",p1);
}
fn gcd(mut a :u64 , mut b:u64)-> u64 {
    while a!=0 {
        if a > b{
            let c  = b;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}
fn multiple_return_values(flag: bool) -> bool {
    if flag == true {
        true
    }else {
        false
    }
}
