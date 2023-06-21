fn main() {
    let x  = 5; //immutable varrable
    println!("The value of x is: {}",x);
    
    let mut y = 10; //mutable varrable
    println!("The value of y is: {}",y);
    y = x;
    println!("The value of y is: {}",y);
    
    const SECONDS: i8 = 60; //const will live as long as programm is running 
    
    println!("The value of SECONDS is: {}",SECONDS);
}

