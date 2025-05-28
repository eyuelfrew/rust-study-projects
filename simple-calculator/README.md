## Simple Calculator Explanation

This Rust program is an interactive command-line calculator that allows users to add two numbers together. Here is a breakdown of how the code works, step by step:

---

### 1. Importing the Standard IO Library

```rust
use std::io;
```
- This line imports Rust's built-in input/output library, which allows the program to read user input from the terminal.

---

### 2. Main Function

```rust
fn main() {
    // ... code ...
}
```
- The `main` function is the entry point of every Rust program. The code inside `main` will be executed when you run the program.

---

### 3. Welcome Message

```rust
println!("🦀 Welcome to the simpleest simple calculator! ");
```
- Prints a welcome message (with a crab emoji, Rust's mascot) to the terminal.

---

### 4. Prompt for the First Number

```rust
println!("Enter the first number: ");
```
- Asks the user to enter the first number.

---

### 5. Read and Parse the First Number

```rust
let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Faild to read line(input)!");
let num1: f64 = input1.trim().parse().expect("Please enter a number");
```
- `let mut input1 = String::new();` creates a mutable String variable to store the user's input.
- `io::stdin().read_line(&mut input1)` waits for the user to type a line and press Enter, then stores that input in `input1`.
    - `.expect("Faild to read line(input)!")` will display an error message if reading input fails.
- `input1.trim().parse()` removes any whitespace and tries to convert the input to a floating-point number (`f64`).
    - `.expect("Please enter a number")` will display an error if parsing fails (i.e., if the user didn't enter a valid number).

---

### 6. Prompt for the Second Number

```rust
println!("Enter second number: ");
```
- Asks the user for the second number.

---

### 7. Read and Parse the Second Number

```rust
let mut input2 = String::new();
io::stdin().read_line(&mut input2).expect("Faild to read line(input)!");
let num2: f64 = input2.trim().parse().expect("Please enter a number!");
```
- Works exactly like the first number:
    - Creates a new mutable String for input.
    - Reads the line from the user.
    - Trims whitespace and parses it into a `f64` number.

---

### 8. Perform Addition

```rust
let sum = num1 + num2;
```
- Adds the two numbers together and stores the result in the variable `sum`.

---

### 9. Output the Result

```rust
println!("The sum is : {}", sum); 
```
- Prints the result of the addition to the terminal.

---

## Example Run

```
🦀 Welcome to the simpleest simple calculator!
Enter the first number:
5.4
Enter second number:
2.3
The sum is : 7.7
```

---

## Notes

- This calculator currently only supports addition of two numbers.
- The program expects valid numerical input. If a user enters something that is not a number, the program will display an error and exit.
- All input is read as strings and then parsed to floating-point numbers (`f64`) for calculation, which allows for both integer and decimal numbers.
- The code uses basic Rust features suitable for beginners.

---
