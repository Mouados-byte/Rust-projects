use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

enum Actions {
    Add { a: f32, b: f32 },
    Substract { a: f32, b: f32 },
    Multiply { a: f32, b: f32 },
    Divide { a: f32, b: f32 },
    Help,
    Quit,
    Unknown,
}

impl Actions {
    fn act(&self) -> bool {
        match self {
            Actions::Add { a, b } => {
                println!("{}", a + b);
                true
            }
            Actions::Substract { a, b } => {
                println!("{}", a - b);
                true
            }
            Actions::Multiply { a, b } => {
                println!("{}", a * b);
                true
            }
            Actions::Divide { a, b } => {
                if *b != 0.0 {
                    println!("{}", a / b);
                } else {
                    println!("The dividor should be a non null value")
                }
                true
            }
            Actions::Help => {
                println!(
                    "Available commands:
                    help - Show this help
                    add <a> <b> - Add two numbers
                    subtract <a> <b> - Subtract b from a
                    multiply <a> <b> - Multiply two numbers
                    divide <a> <b> - Divide a by b
                    quit - Exit the program"
                );
                true
            }
            Actions::Quit => {
                println!("GoodBye!");
                false
            }
            Actions::Unknown => {
                println!("This command is unknown, check the command help to know more");
                true
            }
        }
    }
}

fn parse_command(args: &str) -> Actions {
    let args_list: Vec<&str> = args.trim().split_whitespace().collect();

    match args_list.as_slice() {
        ["Add", a_value, b_value] | ["add", a_value, b_value] => Actions::Add {
            a: a_value.parse::<f32>().unwrap_or(0.0),
            b: b_value.parse::<f32>().unwrap_or(0.0),
        },
        ["Substract", a_value, b_value] | ["sub", a_value, b_value] => Actions::Substract {
            a: a_value.parse::<f32>().unwrap_or(0.0),
            b: b_value.parse::<f32>().unwrap_or(0.0),
        },
        ["Multiply", a_value, b_value] | ["mul", a_value, b_value] => Actions::Multiply {
            a: a_value.parse::<f32>().unwrap_or(0.0),
            b: b_value.parse::<f32>().unwrap_or(0.0),
        },
        ["Divide", a_value, b_value] | ["div", a_value, b_value] => Actions::Divide {
            a: a_value.parse::<f32>().unwrap_or(0.0),
            b: b_value.parse::<f32>().unwrap_or(0.0),
        },
        ["Help"] | ["?"] => Actions::Help,
        ["Quit"] | ["q"] => Actions::Quit,
        _ => Actions::Unknown,
    }
}

fn main() {
    println!("Simple Command Interpreter");
    println!("Type 'help' for available commands");
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut running = true;
    let mut all_commands: Vec<String> = vec![];

    while running {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        all_commands.push(input.clone());

        let command = parse_command(&input);
        running = command.act()
    }
}
