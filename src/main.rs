use std::io;
use std::cmp::Ordering;
use rand::Rng;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "guessing", about="Guessing ")]
struct Opt {

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Maximum number of tries available to the player
    /// The default number is 10
    #[structopt(short = "t", long = "tries", default_value="10")]
    tries: u32,

    /// Lower random number
    #[structopt(short = "l", long = "lower", default_value="1")]
    lower: u32,

    /// Upper random number
    #[structopt(short = "u", long = "upper", default_value="100")]
    upper: u32,
}

fn main() {
    let opt = Opt::from_args();
    println!("+--------------------+");
    println!("|  GUESS THE NUMBER  |");
    println!("+--------------------+");

    let secret_number = rand::thread_rng().gen_range(opt.lower, opt.upper+1);
    let max_tries = opt.tries;
    let mut tries = 0;
    let max_possible_guesses: u32  = ((opt.upper+1) - opt.lower) / 2;

    if opt.debug {
        println!("This game will have a maximum of {} tries! Good luck... ", max_tries);
        println!("This game will have a random number between {} and {}. ", opt.lower, opt.upper);
        println!("max number of guesses can be: {}", max_possible_guesses);
    }


    if max_tries >= max_possible_guesses {
        println!("Maximum number of tries must be less than half the total numbers in the range.");
        std::process::exit(exitcode::OK);
    }

    while tries < max_tries {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };  
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                tries += 1;
                println!("... You have {} tries left ...", max_tries - tries);
            } 
            Ordering::Greater =>  {
                println!("Too big!");
                tries += 1;
                println!("... You have {} tries left ...", max_tries - tries);
            }
            Ordering::Equal => {
                println!("You win!!");
                if tries == 0 {
                    println!("You did it on the first try!! Well done!!");
                } else {
                    println!("You did it in {} tries!", tries + 1);
                }
                break;
            } 
        }
    }
    
   if tries == max_tries {
    println!("You reached the maximum number of tries! The actual number was {}!! Goodbye", secret_number);
   } 
   std::process::exit(exitcode::OK);

}
