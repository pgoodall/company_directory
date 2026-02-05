fn main() {
    use company_directory::employees;
    use company_directory::prompt;

    const GREETING: &str = "
Company Directory
-----------------
Choose from one of the options below:
a) Add an employee to a department
e) Edit an existing employee record
l) List all employees
d) List employees by department
q) Quit ";

    println!("{GREETING}");

    let choice = prompt::get_input("Choice");

    match choice.as_str() {
        "a" => employees::add_employee(),
        "e" => employees::edit_employee(),
        "l" => employees::list_all_employees(),
        "d" => employees::list_by_department(),
        "q" => println!("Goodbye."),
        _ => println!("Please enter a valid choice")
        }

}
