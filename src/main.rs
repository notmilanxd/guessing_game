use std::io;

fn main() {
    println!("not milan original game: Guess the number digga!");

    println!("Please input your guess 🙉");

    let mut guess = String::new(); //mut makes it mutable aka it can be changed (wow)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line hahah");

    println!("You guessed: {guess}");

}
