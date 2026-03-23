fn main() {
    let name = "akhil";
    println!("My first name is {}", name);
    let mut x = 6;
    println!("And i was born on {}th of october", x);
    let month = "october";
    println!("And i was born on {}th of {}", x, month);
    x = 9;
    println!("{} the value of x is modified using mut keyword", x);
}
