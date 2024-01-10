use rand::Rng;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("please input your no");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("couldnt get it");
    println!("you guessed {}", number);
    println!("the number was {}", random_number);
}
