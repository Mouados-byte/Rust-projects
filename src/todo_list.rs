use std::collections::HashMap;
use std::io;
use std::fmt;

struct Todo {
    id: i32,
    description: String,
    tags: Vec<String>,
    completed: bool
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_completed = if self.completed { "X" } else { " " };
        write!(f, "{} [{}] - {}", self.id, is_completed, self.description)?;
        write!(f, "\nTags: {}", self.tags.join(", "))
    }
}

#[derive(Default)]
struct TodoList{
    todos: HashMap<i32, Todo>
}

impl TodoList{
    fn list(&mut self){
        for (_id, todo) in &self.todos {
            println!("{todo}")
        }
    }

    fn add(&mut self, id: i32, description: String, tags: Vec<String>){
        let todo = Todo { id, description, tags, completed: false };
        self.todos.insert(id, todo);
    }

    fn completed(&mut self, id: i32){
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.completed = true
        }
    }

    fn remove(&mut self, id: i32){
        if let Some(_) = self.todos.get(&id){
            self.todos.remove(&id);
        }
    }

    fn filter(&mut self, tag: String) -> (){
        for todo in self.todos.values(){
            if todo.tags.contains(&tag.trim().to_string()){
                println!("{todo}");
            }

        }
    }

    fn parse_command(&mut self, command : &str) -> bool {
        let parsed_command: Vec<&str> = command.split(" ").collect();
        let current_command = parsed_command[0];
        let mut tags: Vec<String> = Vec::new();
        let mut body = String::new();
        
        if parsed_command.len() > 1 {
            tags = parsed_command[parsed_command.len() - 1]
                .split(",")
                .map(|tag| tag.to_string())
                .collect();
            
            if parsed_command.len() > 2 {
                body = parsed_command[1..parsed_command.len()-1]
                    .join(" ")
                    .trim()
                    .to_string();
            }else{
                body = parsed_command[parsed_command.len() - 1].to_string();
            }
        }
        match current_command {
            "list" => {
                self.list();
                true
            },
            "add" | "a" => {
                let id = self.todos.keys().last().unwrap_or(&0) + 1;
                self.add(id, body, tags);
                true
            },
            "remove" | "r" => {
                self.remove(body.parse::<i32>().unwrap());
                true
            },
            "complete" | "c" => {
                let id = body.parse::<i32>().unwrap();
                self.completed(id);
                true
            },
            "filter" | "f" => {
                self.filter(body);
                true
            },
            "quit" | "q" => {
                false
            },
            "help" | "?" => {
                println!("
                    Todo List Manager
            
                    Commands:
                        add <description> [tag1,tag2,...] - Add a new task
                        list - List all tasks
                        complete <id> - Mark a task as completed
                        remove <id> - Remove a task
                        filter <tag> - Filter tasks by tag
                        quit - Exit the program
                ");
                true
            },
            _ => {
                println!("Command not found, try again");
                true
            }
        }
    }
}


fn main(){
    let mut running = true;
    let mut manager: TodoList = TodoList { todos: HashMap::default() };

    println!("
        Todo List Manager

        Commands:
            add <description> [tag1,tag2,...] - Add a new task
            list - List all tasks
            complete <id> - Mark a task as completed
            remove <id> - Remove a task
            filter <tag> - Filter tasks by tag
            quit - Exit the program
    ");
    while running{
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        running = manager.parse_command(input.as_str().trim());
    }
}