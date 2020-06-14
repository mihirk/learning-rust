#[macro_use]
extern crate tramp;

use std::cmp::Ordering;
use std::io::{Read, stdin};

use rand::Rng;
use tramp::{Rec, tramp};

fn read_guess() -> i32 {
    fn read_guess_rec() -> Rec<i32> {
        println!("please input your guess");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Some issue bruv");
        guess
            .trim()
            .parse::<i32>()
            .map(|v| rec_ret!(v))
            .unwrap_or_else(|_| {
                println!("Not a valid integer, please try again");
                rec_call!(read_guess_rec())
            })
    }
    tramp(read_guess_rec())
}


fn main() {
    let generated_number: i32 = rand::thread_rng().gen_range(1, 101);
    println!("guess the number");

    loop {
        match read_guess().cmp(&generated_number) {
            Ordering::Less => println!("higher"),
            Ordering::Greater => println!("lower"),
            Ordering::Equal => break
        }
    }
    println!("It's done innit");
}
