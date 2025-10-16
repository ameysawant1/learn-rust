use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let num: u32 = rng.random_range(1..=100); // use inclusive range for 1–100

    println!("Guess the number!");
    println!("Please input your guess: ");
    
    let hint = if num % 2 == 0 { "even" } else { "odd" };
    println!("Hint: The number is {}.", hint);

    let tip = if num <= 50 { "less than or equal to 50" } else { "greater than 50" };
    println!("Tip: The number is {}.", tip);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the guess");

    let guess: u32 = guess.trim().parse().expect("Please enter a valid number!");

    println!("You guessed: {}", guess);



    if guess == num {
        println!("You win! 🎉");
    } else if guess > num {
        println!("Too high! 😓 The number was: {}", num);
    } else {
        println!("Too low! 😓 The number was: {}", num);
    }

    println!("Thanks for playing! 👋");
}
