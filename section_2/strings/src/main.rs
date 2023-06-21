fn main() {
    //Strings
    let name = String::from("Cristian");
    let course = "Rust".to_string();
    let new_name = name.replace("Cristian", "Cris");
    println!("{}",name);
    println!("{}",course);
    println!("{}",new_name);
    println!("{}",name);

    //&str - string slice || stir
    let str1 = "hello";//creating a slice
    let str2 = str1.to_string();//converting slice to string
    let str3 = &str2;//converting string back to slice
    println!("{}",str1);
    println!("{}",str2);
    println!("{}",str3);

    //== to validate string  || != not equal
    println!("{}", "ONE".to_lowercase() == "one");
    println!("{}", "ONE".to_lowercase() != "one");
}
