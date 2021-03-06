extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字を推測しよう！");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密の数が出力されました！予測してみてください");

    loop {
        println!("推測する数を入力する．");
        let mut guess_number = String::new();
    
        io::stdin().read_line(&mut guess_number)
            .expect("Failed to read line");
    
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess_number);
    
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("数が小さすぎます！"),
            Ordering::Greater => println!("数が大きすぎます！"),
            Ordering::Equal => {
                println!("一致しました！");
                break;
            },
        }
    }
}
