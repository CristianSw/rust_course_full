fn main() {
   let _var = 1 ;// allocated on the stack
    let mut s = "hello".to_string();//allocated on the heap
    s.push_str(",world");
// move
    let x = vec!["move".to_string()];
    let y = x;
   // println!("{:?}",x); violates borrow check ownership from x is given to y and after that we couldnt reference to x because rights are given to y
 println!("{:?}",y);

 // clone
 let x = vec!["clone".to_string()];
 let y = x.clone();
 println!("{:?}",x);
 println!("{:?}",y);

 //copy is implemented on types that are stored on the stack
 let a = 1;
 let b = a;
 println!("a = {}, b = {}", a, b);

 let s = String::from("takes");//create a string with value takes
 take_ownership(s);//give ownership of s to function

 let v = 1;
 make_copy(v);

 let given = give_ownership();
 println!("{}",given);

 let str1:String = take_and_give(given);
 println!("{}",str1);
 //println!("{}",given);


 //references and borrowing
 let mut str1 = String::from("hello");
 change_string(&mut str1);
 println!("{}",str1);

}

fn change_string(str: &mut String){
 str.push_str(",world");
}

fn take_ownership(s: String){
 let x = s;
 println!("{}",x);
}
fn make_copy(one: i32){
 let x = one;
 println!("{}",x);
}
fn give_ownership() -> String{
 "some string".to_string()
}
fn take_and_give(str: String) -> String{
 str
}

//var, s is dropped
