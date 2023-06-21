fn main() {
    //Arrays: basic difference from tuples is that arrays is as collection of data of the same type
    let array = [1,2,3];
    println!("{}", array[0]);
    let array2: [i32; 3] = [4,5,6];
    println!("{}", array2[0]);
    let mut array3: [i32; 3] = [4,5,6];
    println!("{}", array3[0]);
    array3[0] = 10;
    println!("{}", array3[0]);
}
