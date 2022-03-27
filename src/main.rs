use std::collections::HashMap;
use std::io::stdin;
use text_colorizer::*;

type Company = HashMap<String, Vec<String>>;

enum Commands {
    Get,
    Add,
}

fn main() {
    let mut company: Company = HashMap::new();
    println!("{}", "Welcome!".green().bold());

    loop {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        let args = input.split_whitespace().collect();

        let command = if let Some(command) = get_command(&args) {
            command
        } else {
            eprintln!(
                "{}\n{}\nadd employee to department\nget department",
                "Error parsing args!".red().bold(),
                "Examples:".green()
            );
            continue;
        };

        match command {
            Commands::Add => {
                let (employee, department) = (args[1].to_string(), args[3].to_string());
                company
                    .entry(department)
                    .or_insert(Vec::new())
                    .push(employee);
                println!("{}", "Success".green().bold());
            }
            Commands::Get => {
                let department = args[1].to_string();
                let result = if let Some(employees) = company.get(&department) {
                    employees
                } else {
                    eprintln!("{}", "Department not found!".red().bold());
                    continue;
                };
                println!("{:?}", result);
            }
        }
    }
}

fn get_command(args: &Vec<&str>) -> Option<Commands> {
    match args.len() {
        2 => {
            if args[0] != "get" {
                return None;
            }
            Some(Commands::Get)
        }
        4 => {
            if args[0] != "add" {
                return None;
            }
            Some(Commands::Add)
        }
        _ => None,
    }
}
