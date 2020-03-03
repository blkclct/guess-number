extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("数字を推測しよう！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密の数: {}", secret_number);

    println!("推測する数を入力する．");
    let mut guess_number = String::new();

    io::stdin().read_line(&mut guess_number)
        .expect("Failed to read line");

    println!("You guessed: {}", guess_number);
}
