pub mod departments {
    use std::collections::HashMap;
    pub fn list() -> HashMap<i32, String> {
        let mut d_map: HashMap<i32, String> = HashMap::new();

        let departments: Vec<String> = vec![String::from("Engineering"), 
                                        String::from("Sales"), 
                                        String::from("Marketing"), 
                                        String::from("Product"), 
                                        String::from("Legal"), 
                                        String::from("Customer Success")];
        
        // Build a HashMap in order to choose department
        let mut count: i32 = 1;
        for d in departments {
            d_map.insert(count, String::from(d));
            count += 1;
        }

        return d_map

    }
}

pub mod employees {
    use std::collections::{HashMap, btree_set::Difference};

    pub fn add_employee(directory: HashMap<String, String>) -> HashMap<String, String> {
        use crate::prompt;
        use crate::departments;
        use std::collections::HashMap;

        println!("\nAdding an employee to a department");
        println!("----------------------------------");

        println!("Choose a department from the options below:");
        let departments_list: HashMap<i32, String> = departments::list();
        let mut dir: HashMap<String,String> = directory;

        //The HashMap is returned in random order, so I use this to order the output
        let mut count: i32 = 1;
        while (count as usize) < departments_list.len() {
            if let Some(row) = departments_list.get_key_value(&count) {
                println!("{:?}) {}", row.0, row.1);
            }

            count += 1
        }
        
        let choice: i32 = prompt::get_input("Choice").parse().unwrap();
        
        let department:&Option<&String> = &departments_list.get(&choice);
        if let Some(department) = department {
            let name: String = prompt::get_input("Name").parse().unwrap();
            let department: String = department.parse().unwrap();
            dir.entry(department).or_insert(name);
        } 

        return dir

    }

    pub fn edit_employee() {
        println!("Editing an employee...");
    }

    pub fn list_all_employees(directory: &HashMap<String, String>) {
        for row in directory {
            println!("{row:?}");
        }
    }

    pub fn list_by_department(directory: &HashMap<String, String>) {
        for department in directory {
            if let Some(row) = directory.get_key_value(&department) {
                println!("| {} | {} |", row.0, row.1);
            }
        }
    }

}

pub mod prompt {
    pub fn get_input(p: &str) -> String {
        loop {
            use std::io::{self, stdout, Write};
            let mut choice = String::new();

            let mut lock = stdout().lock();
            write!(lock, "{p}: ").unwrap(); // Might need to trap errors here
            lock.flush().expect("Failed to flush stdout.");
        
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read choice.");

            let choice: String = match choice.trim().parse() {
                Ok(choice) => choice,
                Err(_) => continue,
            };

            return choice;
        }
    }
}