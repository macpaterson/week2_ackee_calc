// a program where the user provides 2 numbers and an operator: a and b - and the basic calculator operations are evalauted using them. (add, subtract, multiply, and divide)
// the program will continue looping and let the user keep making operations until the user inputs something that makes the program panic.
use std::io;



fn main() {
    println!("welcome to my first rust calculator B)!");

    
    loop {
// let mut ___ allows the variables on lines 17, 21, and 25 to be borrowed mutably, and lets the read_line fn modify them by the user input and be written into the match pattern.    
        let mut num_1 = String::new();
        let mut num_2 = String::new(); 
        let mut operator = String::new();   

        println!("Enter the first number: ");
        io::stdin().read_line(&mut num_1).expect("failed to read the line. run cargo to try again");
        let num_1: f32 = num_1.trim().parse().expect("Please enter a number next time.");
        
        println!("Enter the second number: ");   
        io::stdin().read_line(&mut num_2).expect("failed to read the line. run cargo to try again");
        let num_2: f32 = num_2.trim().parse().expect("Please enter a number next time.");
      
        println!("What operator would you like to perform? (+, -, *, /) ");
        io::stdin().read_line(&mut operator).expect("failed to read the line. run cargo to try again");
        let operator: char = operator.trim().chars().next().unwrap();
        
        if !"+-*/".contains(operator) {
            println!("Unknown operator. Try using +, -, *, or /.");
            continue;
        }
// evaluate the operator
        let result = match operator {
            '+' => num_1 + num_2,
            '-' => num_1 - num_2,
            '*' => num_1 * num_2,
            '/' => num_1 / num_2,
// this should never happen because of the if check above, but match pattern must be exhausted cuz Rust.
            _ => panic!("Unknown operator"), 
        };

 // display the result with the numbers and operators to reach the result.
        println!("the results of {} {} {} : {}", num_1, operator, num_2, result);
    }
}
