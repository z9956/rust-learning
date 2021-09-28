use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number");

    let num = rand::thread_rng().gen_range(1..100);

    loop {
        println!("please input your guess");

        //mut 使变量可变(Rust中，变量默认是不可变的)
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(..) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&num) {
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("win");
                break;
            }
            Ordering::Greater => println!("big"),
        }
    }
}
