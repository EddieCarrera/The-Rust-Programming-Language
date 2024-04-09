/***************************** Important Notes *********************************
 * 1) read_line puts whatever the user enters into the string we pass to it, but 
 * it also returns a Result value.  If this instance of Result is an Err value, 
 * expect will cause the program to crash and display the message that you passed 
 * as an argument to expect. 
 * 
 * 2) If you don’t call expect, the program will compile, but you’ll get a warning.
 * Rust warns that you haven’t used the Result value returned from read_line, 
 * indicating that the program hasn’t handled a possible error. he right way to 
 * suppress the warning is to actually write error-handling code, but in our case 
 * we just want to crash this program when a problem occurs, so we can use expect.
 * 
 * 3) Ensuring Reproducible Builds with the Cargo.lock File
 * Cargo has a mechanism that ensures you can rebuild the same artifact every time 
 * you or anyone else builds your code: Cargo will use only the versions of the 
 * dependencies you specified until you indicate otherwise. When you build a project 
 * for the first time, Cargo figures out all the versions of the dependencies that 
 * fit the criteria and then writes them to the Cargo.lock file. When you build your 
 * project in the future, Cargo will see that the Cargo.lock file exists and will use 
 * the versions specified there rather than doing all the work of figuring out versions 
 * again.
 * 
 * When you do want to update a crate, Cargo provides the command update, which will
 * ignore the Cargo.lock file and figure out all the latest versions that fit your 
 * specifications in Cargo.toml.
 * 
 * 4) A match expression is made up of arms. An arm consists of a pattern to match 
 * against, and the code that should be run if the value given to match fits that 
 * arm’s pattern. Rust takes the value given to match and looks through each arm’s 
 * pattern in turn. Patterns and the match construct are powerful Rust features: they 
 * let you express a variety of situations your code might encounter and they make 
 * sure you handle them all.
 * 
 * 5) Functions have enums with Ok and Err that contains the value relevant to it.
 *******************************************************************************/

use std::io;
use rand::Rng;
use std::cmp::Ordering; // Provides enums Less, Greater, and Equal

fn main()
{
    /* rand::thread_rng function that gives us the particular random number 
       generator we’re going to use: one that is local to the current thread of 
       execution and is seeded by the operating system */

    /* The kind of range expression we’re using here takes the form start..=end 
       and is inclusive on the lower and upper bounds, so we need to specify 1..=100 
       to request a number between 1 and 100. */
    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess).expect("Failed to read line");
    
    
        /* Rust allows variables to be shadowed, which lets us reuse the guess variable 
        rather than forcing us to create two unique variables (guess_str and guess). */
    
        /* The trim method on a String instance will eliminate any whitespace at the 
        beginning  and end, which we must do to be able to compare the string to the u32, 
        which can only contain numerical data. */
    
        /* The parse method on strings converts a string to another type. Here, we use 
        it to convert from a string to a number. We need to tell Rust the exact number 
        type we want by using let guess: u32. The colon (:) after guess tells Rust we’ll 
        annotate the variable’s type.*/

        // This will cause a crash on error
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        /* If parse returns OK, the match expression will just return the num value that 
        parse produced and put inside the OK value. That number will end up right where we 
        want it in the new guess variable we’re creating. */

        /* The underscore, _, is a catchall value; in this example, we’re saying we want to 
        match all Err values, no matter what information they have inside them.   */
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue            
        };
    
        println!("You guessed: {guess}");
    
        /* The cmp method compares two values and can be called on anything that can 
        be compared. It takes a reference to whatever you want to compare with. Then it 
        returns a variant of the Ordering enum we brought into scope with the use 
        statement. */
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break out of loop
            }
        }
    } // loop
}