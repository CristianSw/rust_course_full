fn main() {

    //integers can be : 8,16,31,64,128 bit
    let x: i8 = 10;//implicit declaration
    println!{"This is 8bit int x: {}", x};
    let x1: i16 = 10;//implicit declaration
    println!("This is 16bit int x: {}", x1);
    let x2: i32 = 10;//implicit declaration
    println!("This is 32bit int x: {}", x2);
    let x3: i64 = 10;//implicit declaration
    println!("This is 64bit int x: {}", x3);
    let x4: i128 = 10;//implicit declaration
    println!("This is 128bit int x: {}", x4);
    //integers can be signed and unsigned
    let y: u8 = 10;//implicit declaration
    println!("This is 8bit unsigned int y: {}", y);
    //integer literals
    let decimal = 02_55;//explicit declaration
    let hex = 0xff;//explicit declaration
    let octal = 0o377;//explicit declaration
    let binary = 0b1111_1111;//explicit declaration
    println!("decimal value of 02_55: {}", decimal);
    println!("hex value of 0xff: {}", hex);
    println!("octal value of 0o377: {}", octal);
    println!("binary value of 0b1111_1111: {}", binary);
    //byte type
    let byte = b'A';//explicit declaration
    println!("ASCI code of A: {}", byte);
    
    //floats default is f64
    let fl = 0.64; // f is f64 by default 
    let fl1: f32 = 0.32; //implicit declaration
    
    println!("Default float 64bit f: {}",fl);
    println!("float with specified f32 f1: {}",fl1);
    
    //boolean
    let t = true; //explicit declaration
    let f: bool = false;//implicit declaration
    println!("True t: {}",t);
    println!("False f: {}",f);
    
    //char
    let c = 'c';
    let b: char = 'b';//implicit declaration
    println!("{}",c);
    println!("{}",b);
    
    //arithmetic operators: +,-,*,/,%
    let z1 = 20.0;
    let a1 = 234.7566;
    let c = z1 + a1;
    let remainder = c % z1;
    println!("z1: {}", z1);
    println!("a1: {}", a1);
    println!("c: z1 + a1 : {}", c);
    println!("remainder: {}", remainder);
 }