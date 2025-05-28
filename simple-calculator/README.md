
Simple Rust Calculator 🦀
Welcome to the Simple Rust Calculator, a beginner-friendly Rust program that takes two numbers as input from the user and calculates their sum. This project is perfect for learning Rust's basics, including standard input/output, variable handling, and type parsing. Below, you'll find a line-by-line explanation of the code, making it easy to understand each component.
Prerequisites

Rust: Ensure Rust is installed on your Ubuntu system (or any OS). Verify with:rustc --version
cargo --version

Code Explanation

use std::io;

*Purpose: Imports the io module from Rust’s standard library, enabling input/output operations, such as reading from stdin (standard input, i.e., the terminal).
Why: Necessary for functions like stdin().read_line() to capture user input, similar to prompting for DTMF in your FreePBX IVR (April 15, 2025**).


fn main() {

Purpose: Defines the main function, the entry point of the Rust program, like the starting point of an Asterisk dialplan.
Why: Every Rust application begins execution here.


println!("🦀 Welcome to the simpleest simple calculator! ");

Purpose: Prints a welcome message with a crab emoji (🦀), Rust’s mascot, to the terminal using the println! macro.
Why: Provides a friendly greeting, like your IVR’s welcome message (welcome.slin). The ! indicates println! is a macro, not a function.


println!("Enter the first number: ");

Purpose: Prompts the user to enter the first number, displayed in the terminal.
Why: Guides the user to provide input, similar to an IVR saying “Press 1 for sales.”


let mut input1 = String::new();

Purpose: Creates a new, mutable String named input1 to store the user’s input as a string.
let: Declares a variable (binding).
mut**: Makes input1 mutable, allowing it to be modified (e.g., by read_line).
String::new(): Initializes an empty string.
Why: Rust’s type safety requires a string to hold raw input before parsing, like a buffer for DTMF input.


io::stdin().read_line(&mut input1).expect("Faild to read line(input)!");

Purpose: Reads a line of user input from the terminal (stdin) into input1.
io::stdin(): Accesses standard input.
read_line(&mut input1): Appends the input (including newline) to input1.
&mut input1: Passes a mutable reference to input1.
expect("..."): Handles errors (e.g., if input fails) by panicking with the message “Faild to read line(input)!“. Note: “Faild” is a typo (should be “Failed”).


Why: Captures user input, like reading a keypress in your IVR. The expect ensures robust error handling, a Rust hallmark.


let num1: f64 = input1.trim().parse().expect("Please enter a number");

Purpose: Converts the string input1 into a 64-bit floating-point number (f64) and stores it in num1.
trim(): Removes whitespace and the trailing newline (\n) from input1.
parse(): Attempts to parse the string into an f64.
expect("..."): Panics with “Please enter a number” if parsing fails (e.g., if the user enters “abc”).
: f64: Explicitly specifies the type, as Rust requires type clarity.


Why: Converts user input into a number for calculations, like parsing DTMF digits to route calls.


// go for the second number

Purpose: A comment indicating the next section handles the second number.
Why: Improves code readability, like comments in your Asterisk dialplan.


// Second number (same goes here like the first number)

Purpose: A comment explaining the second number input process mirrors the first.
Why: Clarifies intent for readers.


println!("Enter second number: ");

Purpose: Prompts for the second number.
Why: Guides the user, like the first prompt.


let mut input2 = String::new();

Purpose: Creates a mutable String named input2 for the second input.
Why: Same as input1, prepares to store raw input.


io::stdin().read_line(&mut input2).expect("Faild to read line(input)!");

Purpose: Reads the second input into input2.
Why: Identical to the first read_line, captures the second number. Same typo (“Faild”).


let num2: f64 = input2.trim().parse().expect("Please enter a number!");

Purpose: Parses input2 into an f64 named num2.
Why: Converts the second input for calculation, like num1.


// for know let do the addition,

Purpose: A comment indicating the addition operation. Note: “know” should be “now,” and “let” should be “let’s.”
Why: Explains the next step.


let sum = num1 + num2;

Purpose: Adds num1 and num2, storing the result in sum (inferred as f64).
Why: Performs the core calculation, like routing a call after collecting DTMF.


// and print the result in the terminal/cmd

Purpose: A comment explaining the result will be displayed.
Why: Clarifies the final step.


println!("The sum is : {}", sum);

Purpose: Prints the sum to the terminal.
{}: Placeholder for sum.
sum: Inserted into the placeholder.


Why: Shows the result, like an IVR announcing a call destination. The space in “is :” is stylistic.


}

Purpose: Closes the main function.
Why: Completes the program.



How to Run

Save the Code:

Save the code in a file named main.rs in a new Rust project:cargo new simple-calculator
cd simple-calculator
nano src/main.rs

Paste the code, save, and exit.


Compile and Run:
cargo run


Interact:

Enter the first number (e.g., 5.5) and press Enter.
Enter the second number (e.g., 3.2) and press Enter.
See the sum (e.g., The sum is : 8.7).



Notes

Error Handling: The expect calls panic on invalid input. For production, consider graceful error handling (e.g., match or Result).
Typos: Fix “Faild” to “Failed” and “know” to “now” for clarity.
Extensions: Add more operations (e.g., subtraction, multiplication) by extending the code.
Rust Learning: This program showcases Rust’s type safety, ownership, and standard library, perfect for your Rust studies alongside your Tasks CRUD API (May 28, 2025).

Contributing
Feel free to fork this project, add features (e.g., more operations), or fix typos. Submit a pull request or open an issue for suggestions!
License
MIT License. See LICENSE for details.
