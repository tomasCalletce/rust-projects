use std::io::{self, Write};

struct Task {
  id: i32,
  description: String,
  done: bool
}

pub fn run_todo_list(){
  let mut tasks : Vec<Task> = Vec::new();

  println!("Welcome to the TODO-APP");

  loop {
      let mut input = String::new();

      println!("Input a command: add, list, done, exit");
        
      io::stdout().flush().unwrap();
      io::stdin().read_line(&mut input).expect("Failed to read line");
  
      let trimmed_input = input.trim();
  
      if trimmed_input == "exit" {
        break;
      }

      match trimmed_input {
          "add" => add_task(&mut tasks),
          "list" => list_tasks(&tasks),
          "done" => mark_as_done(&mut tasks),
          _  => unreachable!()
      }
  }
  
}

fn mark_as_done(tasks : &mut Vec<Task>){
  println!("Input the id of the task you whant to mark as done");
  let mut input = String::new();
      
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  let target_id : i32 = input.trim().parse().unwrap();

  for task in tasks {
    if target_id == task.id {
      task.done = true;
      println!("Task {} marked as done",task.id);
      return;
    }
  }
  println!("Task not found");
}

fn list_tasks(tasks : &Vec<Task>){
  for task in tasks {
    println!("------------------");
    println!("id: {} description: {} done: {}", task.id, task.description, task.done);
    println!("------------------");
  }
}

fn add_task(tasks : &mut Vec<Task>) {
    println!("Add task with this format key,description");
    let mut input = String::new();
        
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arguments : Vec<&str> = input.split(",").collect();

    if arguments.len() != 2 {
      println!("Invalid input");
      return;
    }

    let task = Task {
      id : arguments[0].parse().unwrap(),
      description: arguments[1].to_string(),
      done: false
    };

    tasks.push(task);
    println!("Task added");
} 