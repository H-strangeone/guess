use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    println!("please input your no");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("couldnt get it");
    let random_number= number.to_string();
    match guess.cmp(&random_number){
       Ordering::Less=> println!("naah fam"),
       Ordering::Equal=> println!("yessir"),
       Ordering::Greater=> println!("still naah fam"),
    }
    println!("you guessed {}", guess);
    println!("the number was {}", random_number);
}
