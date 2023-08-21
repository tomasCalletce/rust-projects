use std::io::{self, Write};

pub fn run_calculator() {
    println!("Hello, world!");

    let mut stack: Vec<i32> = Vec::new();

    loop {
        let mut input = String::new();
        
        println!("Input an operator (+,-) or a number, or exit to quit");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            println!("result = {}",stack.pop().unwrap());
          break;
        }

        match trimmed_input {
            "+" => add(&mut stack),
            "-" => sub(&mut stack),
            _ => {
                if let Ok(num) = trimmed_input.parse::<i32>(){
                  stack.push(num);
                }else {
                  println!("Invalid input")
                }
            }
        }
       
    }
}

fn sub(stack : &mut Vec<i32>){
  let a = stack.pop().unwrap();
  let b = stack.pop().unwrap();
  stack.push(a - b);
}

fn add(stack: &mut Vec<i32>) {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(a + b);
}