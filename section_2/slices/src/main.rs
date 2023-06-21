fn main() {
    //Slices are not owning reference, single reference is pointing to a value in memory, slice is pointing to a range of values in memory
    let ve :Vec<i32> = (0..10).collect();
    println!("{:?}", ve);
    let sve : &[i32] = &ve;//
    println!("{:?}", sve);
}
