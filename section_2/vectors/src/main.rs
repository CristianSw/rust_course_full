fn main() {
    let mut nums = vec![1,2,3];
    nums.push(4);
    println!("{:?}", nums);
    println!("{:#?}", nums);
    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();
    vec.push("String");
    vec.push("Value");
    println!("{:?}", vec);
    vec.reverse();
    println!("{:?}", vec);

    let mut v = Vec::<i32>::with_capacity(2);
    v.push(1234);
    v.push(5);
    v.push(2);
    println!("{}", v.capacity());

    let vector: Vec<i32> = (0..100).collect();
    println!("{:?}", vector);
}



