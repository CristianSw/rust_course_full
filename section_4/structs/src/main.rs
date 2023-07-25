struct User{
    username: String,
    active: bool,
    sign_in_counts: u32,
}
struct Coordinates(i32,i32,i32);
struct UnitStruct;

struct Square{
    width: u32,
    height: u32,
}
impl Square{
    fn area(&self) -> u32{
        self.height * self.width
    }
    fn get_width(&self) -> u32{
        self.width
    }
    fn get_height(&self) -> u32{
        self.height
    }
    fn set_width(&mut self, width: u32){
        self.width = width;
    }
    fn set_height(&mut self, height: u32){
        self.height = height;
    }
}

struct MyString<'a>{
    text: &'a str,
}

struct Car{
    mpg: u32,
    color: String,
    top_speed: u32,
}
impl Car{
    fn set_mpg(&mut self, mpg: u32){
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: String){
        self.color = color;
    }
    fn set_top_speed(&mut self, top_speed: u32){
        self.top_speed = top_speed;
    }
}

fn main() {
    let user1 = User{username: String::from("Cristian"), active: true, sign_in_counts: 0};
    println!("{}", user1.username);
    let mut user2 = build_user(String::from("Test"),true,5);
    println!("{}", user2.username);

    for i in 1..10 {
        user2.sign_in_counts +=1;
    }
    println!("{}", user2.sign_in_counts);
    let cords = Coordinates(1,2,3);
    println!("{}", cords.0);

    //1..5 == Range{start: 1, end: 5}

    let mut square1 = Square{width: 5, height: 5};
    println!("{}", square1.area());
    println!("{}", square1.get_width());
    println!("{}", square1.get_height());

    square1.set_width(14);
    square1.set_height(20);
    println!("{}", square1.area());
    println!("{}", square1.get_width());
    println!("{}", square1.get_height());


    //lifetime
    //&i32
    //&'a i32
    //&'a mut i32

    let strng = String::from("Text");
    let x = MyString{text: &strng};
    println!("{}", x.text);

    //static lifetime live in app binary
    let s : &'static str = "I have static lifetime";
    println!("{}", s);
    let mut car = Car{mpg: 100, color: String::from("Blue"),top_speed: 250};
    car.set_mpg(1000);
    car.set_color("White".to_string());
    car.set_top_speed(350);
    println!("car mpg: {},color: {},top speed: {}", car.mpg,car.color, car.top_speed)
}

fn lifetime<'a, 'b>(x: &'a str, y: &'b str)-> &'a str{
    x
}


fn build_user(username: String, active: bool, sign_in_counts: u32) -> User{
    User{
        username,
        active,
        sign_in_counts,
    }
}
