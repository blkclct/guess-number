use std::io;

fn main() {
    println!("数字を推測しよう！");

    println!("推測する数を入力する．");
    let mut guess_number = String::new();

    io::stdin().read_line(&mut guess_number)
        .expect("Failed to read line");

    println!("You guessed: {}", guess_number);
}
