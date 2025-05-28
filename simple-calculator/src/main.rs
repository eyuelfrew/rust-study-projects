use std::io;

fn main() {
    println!("🦀 Welcome to the simpleest simple calculator! ");

    // First number (write the first number and press enter in the cmd/terminal)
    println!("Enter the first number: ");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Faild to read line(input)!");
    let num1: f64 = input1.trim().parse().expect("Please enter a number");
    // go for the second number 
    
    // Second number (same goes here like the first number)
    println!("Enter second number: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Faild to read line(input)!");
    let num2: f64 = input2.trim().parse().expect("Please enter a number!");
    
    // for know let do the addition, 
    let sum = num1 + num2;

    // and print the result in the terminal/cmd 
    println!("The sum is : {}", sum);


   
    

}

