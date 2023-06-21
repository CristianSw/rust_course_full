fn main() {
    // Tuples once touple is declared it can't grow and shring in size
    let tup1 = (100,"string", true);
    let tup2 = (100,200.99,'c',false);
    let t1 = tup1.0 + tup2.0;
    let t2 = format!("{} + {}",tup1.1,tup2.2);
    let t3 = tup1.2 && !tup2.3;
    let (x,y,z,i) = tup2;
    
    println!("t1: {}",t1);
    println!("t2: {}",t2);
    println!("t3: {}",t3);
    
    println!("x: {}",x);
    println!("y: {}",y);
    println!("z: {}",z);
    println!("i: {}",i);
}
