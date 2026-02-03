/* fn menu_prompt() -> String {
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
} */

fn main() {
    use company_directory::employees;
    use company_directory::prompt;

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

    let choice = prompt::get_input();

    match choice.as_str() {
        "a" => employees::add_employee(),
        "e" => employees::edit_employee(),
        "l" => employees::list_all_employees(),
        "d" => employees::list_by_department(),
        "q" => println!("Goodbye."),
        _ => println!("Please enter a valid choice")
        }

}
