fn main() {
    let one = 1;
    if one > 10 {
        println!("True");
    }else if one == 1{
        println!("Equal");
    }else {
        println!("False");
    }
    //loop analog for while

    let mut i = 0;
    // loop { //infinite loop
    //     println!("{}",i++);
    // }
    'counter : loop {
        println!("Counter i: {}", i);
        let mut d = 5;
        loop{
            if d == 3{
                break;
            }
            if i == 2 {
                break 'counter;
            }
            d -= 1;
        }
        i += 1;
    }

    let mut num = 0;
    while num <= 5 {
        println!("Num: {}", num);
        num += 1;
    }

    let vec :Vec<i8> = (0..10).collect();
    for element in vec {
        println!("{}", element);
    }
}

