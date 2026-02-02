//use std::io::{self, Write};

//use std::{collections::HashMap, ops::Index};

fn menu_prompt() -> String {
    loop {
        use std::io::{self, stdout, Write};
        let mut choice = String::new();

        let mut lock = stdout().lock();
        write!(lock, "Choice: ").unwrap(); // Might need to trap errors here
        lock.flush().expect("Failed to flush stdout.");
    
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");

        let choice: String = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => continue,
        };

        return choice;
    };
}

fn add_employee(departments: &Vec<String>) {
    use std::collections::HashMap;
    
    let mut d_map: HashMap<i32, String> = HashMap::new();

    println!("\nAdding an employee to a department");
    println!("----------------------------------");

    println!("Choose a department from the options below:");
    let mut count: i32 = 1;
    for d in departments {
        println!("{count}) {d}");
        d_map.insert(count, String::from(d));
        count += 1;

    }

    let choice: i32 = menu_prompt().parse().unwrap();
    
    let department:&Option<&String> = &d_map.get(&choice);
    if let Some(department) = department {
        println!("Adding employee to the {department} department.");
    } else {
        // I don't think this branch is ever reached
        println!("Please enter a valid department choice.");
    }
    
}

fn edit_employee() {
    println!("Editing an employee...");
}

fn list_all_employees() {
    println!("Listing all employees by department...");
}

fn list_by_department() {
    println!("List employees in a department...");
}

fn main() {
    const GREETING: &str = "
Company Directory
-----------------
Choose from one of the options below:
a) Add an employee to a department
e) Edit an existing employee record
l) List employees across all departments
d) List employees in a department
q) Quit ";

    println!("{GREETING}");

    let departments: Vec<String> = vec![String::from("Engineering"), 
                                        String::from("Sales"), 
                                        String::from("Marketing"), 
                                        String::from("Product"), 
                                        String::from("Legal"), 
                                        String::from("Customer Success")];

    let choice = menu_prompt();

    match choice.as_str() {
        "a" => add_employee(&departments),
        "e" => edit_employee(),
        "l" => list_all_employees(),
        "d" => list_by_department(),
        "q" => println!("Goodbye."),
        _ => println!("Please enter a valid choice")
        }

}
