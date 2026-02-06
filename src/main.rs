fn main() {
    use company_directory::employees;
    use company_directory::prompt;
    use std::collections::HashMap;

    let mut directory: HashMap<String, String> = HashMap::new();

    const GREETING: &str = "
Company Directory
-----------------
Choose from one of the options below:
a) Add an employee to a department
e) Edit an existing employee record
l) List all employees
d) List employees by department
q) Quit ";

    loop {
        println!("{GREETING}");

        let choice = prompt::get_input("Choice");

        match choice.as_str() {
            "a" => directory = employees::add_employee(directory),
            "e" => employees::edit_employee(),
            "l" => employees::list_all_employees(&directory),
            "d" => employees::list_by_department(&directory),
            "q" => { println!("Goodbye."); break },
            _ => println!("Please enter a valid choice")
            }
    }

}
