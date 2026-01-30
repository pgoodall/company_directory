use std::io::{self, Write};

fn add_employee() {
    println!("Adding an employee...");
}

fn edit_employee() {
    println!("Editing an empoyee...");
}

fn list_employees() {
    println!("Listing employees...");
}

fn main() {
    let greeting: &'static str = "
Company Directory
-----------------
Choose from one of the options below:
a) Add an employee to the directory
e) Edit an existing employee record
l) List the current employees
q) Quit ";

    println!("{greeting}");
        
    loop {
        use std::io::{stdout, Write};

        let mut lock = stdout().lock();
        write!(lock, "Choice: ").unwrap(); // Might need to trap errors here
        lock.flush().expect("Failed to flush stdout.");
        
        let mut choice = String::new();
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");
        
        let choice: char = match choice.trim().parse() {
            Ok(char) => char,
            Err(_) => continue
        };

        match choice {
            'a' => add_employee(),
            'e' => edit_employee(),
            'l' => list_employees(),
            'q' => break,
            _ => println!("Please enter a valid choice")
        }
    }
}
