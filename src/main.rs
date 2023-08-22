use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("not milan original game: Guess the number!");
    println!("You can quit with ctrl+c but why would u do that-");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {

        println!("Please input your guess 🙉");

        let mut guess = String::new(); //mut makes it mutable aka it can be changed (wow)

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line hahah");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! 🗿"),
            Ordering::Equal => {
                println!("✅ Wow you've got it right");
                break;
                }
            Ordering::Greater => println!("Too big! 😩"),
        }
    }
}
