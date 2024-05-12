use rand::Rng;
use std::io;
use std::io::Write;

//this code was generated by ChatGPT to clear the terminal screen on all operating systems
fn clear_screen() {
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("cmd")
        .arg("/c")
        .arg("cls")
        .status();

    #[cfg(not(target_os = "windows"))]
    let _ = Command::new("clear").status();
}

//this code was generated by ChatGPT to read a text file, iterate through, splitting entries on the ", " delimeter and adding each item to a vector
//after seeing a possible solution, I deleted their function and wrote it again myself, asking questions about syntax/error handling when I was stuck
//credits to alpha-tango on github, as they provided the word bank for hangman. I got the data from this link: https://gist.github.com/alpha-tango/c3d2645817cf4af2aa45#file-hangman_wordbank
fn word_bank_creator() -> Vec<String> {
    let mut temp_vector: Vec<String> = Vec::new();
    //saves data from word_bank.txt as data
    let data = include_str!("word_bank.txt");
    // iterates through data, splits on ", ", pushes to a vector, and returns that vector
    for line in data.lines() {
        // let line = line.expect("Failed to read line");
        let entries: Vec<&str> = line.split(", ").collect();
        for entry in entries {
            temp_vector.push(entry.to_string());
        }
    }
    return temp_vector;
}

fn main() {
    // all bold ascii text was generated via this website: https://patorjk.com/software/taag/#p=display&f=Graffiti&t=Type%20Something%20
    let you_win = r#"██╗   ██╗     ██████╗     ██╗   ██╗            ██╗    ██╗    ██╗    ███╗   ██╗    ██╗    ██╗    ██╗    
╚██╗ ██╔╝    ██╔═══██╗    ██║   ██║            ██║    ██║    ██║    ████╗  ██║    ██║    ██║    ██║    
 ╚████╔╝     ██║   ██║    ██║   ██║            ██║ █╗ ██║    ██║    ██╔██╗ ██║    ██║    ██║    ██║    
  ╚██╔╝      ██║   ██║    ██║   ██║            ██║███╗██║    ██║    ██║╚██╗██║    ╚═╝    ╚═╝    ╚═╝    
   ██║       ╚██████╔╝    ╚██████╔╝            ╚███╔███╔╝    ██║    ██║ ╚████║    ██╗    ██╗    ██╗    
   ╚═╝        ╚═════╝      ╚═════╝              ╚══╝╚══╝     ╚═╝    ╚═╝  ╚═══╝    ╚═╝    ╚═╝    ╚═╝"#;
    let welcome = r#"


        ███████╗██████╗ ██╗  ██╗██████╗  █████╗ ██╗███╗   ███╗███████╗    
        ██╔════╝██╔══██╗██║  ██║██╔══██╗██╔══██╗██║████╗ ████║██╔════╝    
        █████╗  ██████╔╝███████║██████╔╝███████║██║██╔████╔██║███████╗    
        ██╔══╝  ██╔═══╝ ██╔══██║██╔══██╗██╔══██║██║██║╚██╔╝██║╚════██║    
        ███████╗██║     ██║  ██║██║  ██║██║  ██║██║██║ ╚═╝ ██║███████║    
        ╚══════╝╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚══════╝    
                                                                          
    ██╗  ██╗ █████╗ ███╗   ██╗ ██████╗ ███╗   ███╗ █████╗ ███╗   ██╗██╗██╗
    ██║  ██║██╔══██╗████╗  ██║██╔════╝ ████╗ ████║██╔══██╗████╗  ██║██║██║
    ███████║███████║██╔██╗ ██║██║  ███╗██╔████╔██║███████║██╔██╗ ██║██║██║
    ██╔══██║██╔══██║██║╚██╗██║██║   ██║██║╚██╔╝██║██╔══██║██║╚██╗██║╚═╝╚═╝
    ██║  ██║██║  ██║██║ ╚████║╚██████╔╝██║ ╚═╝ ██║██║  ██║██║ ╚████║██╗██╗
    ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝╚═╝"#;

    let you_lose = r#"


██╗   ██╗ ██████╗ ██╗   ██╗    ██╗      ██████╗ ███████╗███████╗██╗██╗██╗
╚██╗ ██╔╝██╔═══██╗██║   ██║    ██║     ██╔═══██╗██╔════╝██╔════╝██║██║██║
 ╚████╔╝ ██║   ██║██║   ██║    ██║     ██║   ██║███████╗█████╗  ██║██║██║
  ╚██╔╝  ██║   ██║██║   ██║    ██║     ██║   ██║╚════██║██╔══╝  ╚═╝╚═╝╚═╝
   ██║   ╚██████╔╝╚██████╔╝    ███████╗╚██████╔╝███████║███████╗██╗██╗██╗
   ╚═╝    ╚═════╝  ╚═════╝     ╚══════╝ ╚═════╝ ╚══════╝╚══════╝╚═╝╚═╝╚═╝"#;

    // all of the hangman/gallows art was generated by ChatGPT 3.5
    let gallows = r#"
	  +---+
	  |   |
	      |
	      |
	      |
	      |
	========="#;

    let head = r#"
	  +---+
	  |   |
	  O   |
	      |
	      |
	      |
	========="#;

    let body = r#"
	  +---+
	  |   |
	  O   |
	  |   |
	      |
	      |
	========="#;

    let left_arm = r#"
	  +---+
	  |   |
	  O   |
	 /|   |
	      |
	      |
	========="#;

    let right_arm = r#"
	  +---+
	  |   |
	  O   |
	 /|\  |
	      |
	      |
	========="#;

    let left_leg = r#"
	  +---+
	  |   |
	  O   |
	 /|\  |
	 /    |
	      |
	========="#;

    let right_leg = r#"
	  +---+
	  |   |
	  O   |
	 /|\  |
	 / \  |
	      |
	========="#;

    //define variables

    //define victory/loss bool
    let mut win: bool = false;
    // define variable to hold a guess made after typing solve
    let mut solve_guess = String::new();
    //define an array to hold my ascii art of gallows and various stages of hanged man
    let pictures: [&str; 7] = [
        gallows, head, body, left_arm, right_arm, left_leg, right_leg,
    ];
    //define a variable to hold normal guesses
    let mut guess = String::new();
    //create a vector and give it the value of the temp_vector from the function word_bank_creator
    let mut answer_word_bank: Vec<String> = word_bank_creator();
    //porcupine was the test word, I wanted to add it to the list
    answer_word_bank.push("porcupine".to_string());
    //define a random object and generate a random number to use as an index to assign the hidden word

    let mut rng = rand::thread_rng();
    let answer_index = rng.gen_range(0..answer_word_bank.len());

    //define the hidden word, the answer
    let answer = &answer_word_bank[answer_index];

    // define a vector to hold correct guesses
    let mut correct_guesses: Vec<String> = vec![];
    // define a vector to hold wrong guesses
    let mut incorrect_guesses: Vec<String> = vec![];
    // define a vector to hold all guesses
    let mut all_guesses: Vec<String> = vec![];
    //define a bool make the game loop continue
    let mut keep_going: bool = true;

    //clear the screen
    clear_screen();
    //display main menu text
    println!("{}", welcome);

    while keep_going {
        //have the user press enter to start the program/every loop to preserve what was just printed onto the screen
        println!("Press enter to continue ");
        {
            let mut _temporary_holder = Default::default();
            io::stdin().read_line(&mut _temporary_holder).unwrap();
        }

        //clear the screen
        clear_screen();

        //make it look cool
        println!("{}", pictures[incorrect_guesses.len()]);

        //Inform the user of the game
        println!("Guess the word hidden by the blanks");
        println!("If you think you know the word, type 'solve'");

        //display what characters of the word that the user has guessed
        for letter in answer.chars() {
            if correct_guesses.contains(&letter.to_string()) {
                print!("{} ", letter);
            } else {
                print!("_ ");
            };
        }

        println!("\n");

        //display list of already guessed letters
        if incorrect_guesses.len() > 0 {
            println!("Your incorrect guesses are: ");
            for incorrect_guess in &incorrect_guesses {
                if incorrect_guesses[incorrect_guesses.len() - 1] == incorrect_guess.to_string() {
                    println!("{} \n", incorrect_guess);
                } else {
                    print!("{}, ", incorrect_guess);
                }
            }
        } else {
            println!("You have no incorrect guesses so far")
        }

        //get an input from the user. verify that this guess is unique
        println!("Guess a character!");
        print!("Please input your guess: ");

        //this line clears the standard output stream, fixing a bug with the print! macro. I found this code at https://stackoverflow.com/questions/77197962
        io::stdout().flush().unwrap();
        //get an input from the user and trim their input
        let _ = io::stdin().read_line(&mut guess);
        guess = guess.trim().to_string();
        println!();

        //verify that this guess has not been made
        if all_guesses.contains(&guess) {
            println!(
                "You have already guessed '{}'. Make a different guess.",
                &guess
            );
            guess = String::new();
            continue;
        }
        if guess == "debugging break" || guess == "debugging clear" || guess == "solve" {
            //debugging functionality
            if guess == "debugging break" {
                break;
            }
            //debugging functionality
            if guess == "debugging clear" {
                incorrect_guesses.clear();
                correct_guesses.clear();
                guess.clear();
                continue;
            }

            //functionality to guess the word early
            if guess == "solve" {
                print!("Enter your guess as to what the entire word is: ");
                //this line clears the standard output stream, fixing a bug/interaction with the print!() macro. I found this code at https://stackoverflow.com/questions/77197962
                io::stdout().flush().unwrap();
                let _ = io::stdin().read_line(&mut solve_guess);

                //trims solve-specific guess, and lets you win if you solve it correctly
                solve_guess = solve_guess.trim().to_string();
                if solve_guess == *answer {
                    win = true;
                    break;
                }
                //copy solve_guess to guess so that it can be added to the list of wrong guesses
                guess = solve_guess.clone();
            }
        } else if guess.len() != 1 || guess == "\n" {
            println!(
                "{} is not a valid guess. Valid guesses are 1 character long OR 'solve'",
                &guess
            );
            guess = String::new();
            continue;
        }

        //check if guess is part of answer

        if answer.contains(&guess) {
            //if true, display message and add value of guess to vector of correct guesses
            println!("Your guess was correct!");
            correct_guesses.push(guess.clone());

            //check to see if player has guessed all of the necessary letters and wins
            {
                let mut solved_word = String::new();
                for letter in answer.chars() {
                    if correct_guesses.contains(&letter.to_string()) {
                        solved_word = solved_word + &letter.to_string();
                    }
                }
                if solved_word == *answer {
                    win = true;
                    keep_going = false;
                }
            }
        } else {
            //if not true, display message and add to vector of incorrect guesses
            println!("Your guess was incorrect!");
            incorrect_guesses.push(guess.clone());

            //check to see if the man is fully hanged and the player loses
            if incorrect_guesses.len() >= 6 {
                win = false;
                keep_going = false;
            }
        };
        //clear value of guess and vector of all guesses
        guess = String::new();
        all_guesses.clear();
        //add all items from vector of correct and incorrect guesses to vector of all guesses
        all_guesses = correct_guesses
            .iter()
            .cloned()
            .chain(incorrect_guesses.iter().cloned())
            .collect();
    }
    if win {
        //clear the screen
        clear_screen();
        //display victory text
        println!("{}", you_win);
        //their hanged man
        println!("{}\n", pictures[incorrect_guesses.len()]);
        println!("The word was {}\n", &answer);

        //display wrong user guesses
        if incorrect_guesses.len() > 0 {
            println!("Your incorrect guesses were: ");
            for incorrect_guess in &incorrect_guesses {
                if incorrect_guesses[incorrect_guesses.len() - 1] == incorrect_guess.to_string() {
                    println!("{} \n", incorrect_guess);
                } else {
                    print!("{}, ", incorrect_guess);
                }
            }
        } else {
            println!("Wow! You had no incorrect guesses!\n")
        }

        //have the user press enter to exit
        println!("Press enter to exit");
        {
            let mut _temporary_holder = Default::default();
            io::stdin().read_line(&mut _temporary_holder).unwrap();
        }
    } else {
        //clear the screen
        clear_screen();
        //display loss text
        println!("{}", you_lose);
        println!("{}\n", pictures[6]);
        //display what the word was
        println!("The word was {}\n", answer);
        //have the user press enter to exit
        println!("Press enter to exit\n");
        {
            let mut _temporary_holder = Default::default();
            io::stdin().read_line(&mut _temporary_holder).unwrap();
        }
    }
}