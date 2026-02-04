// ----------------------------------------------------------- //
// Switch departments to IndexMap to make it easier to sort?   //
// ----------------------------------------------------------- //

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
    pub fn add_employee() {
        use crate::prompt;
        use crate::departments;
        use std::collections::HashMap;

        println!("\nAdding an employee to a department");
        println!("----------------------------------");

        println!("Choose a department from the options below:");
        let departments_list: HashMap<i32, String> = departments::list();

        //The HashMap is returned in random order, so I use this to order the output
        let mut count: i32 = 1;
        while (count as usize) < departments_list.len() {
            if let Some(row) = departments_list.get_key_value(&count) {
                println!("{:?}) {}", row.0, row.1);
            }

            count += 1
        }
        
        let choice: i32 = prompt::get_input().parse().unwrap();
        
        let department:&Option<&String> = &departments_list.get(&choice);
        if let Some(department) = department {
            println!("Adding employee to the {department} department.");
        } else {
            // I don't think this branch is ever reached
            println!("Please enter a valid department choice.");
        }

    }

    pub fn edit_employee() {
        println!("Editing an employee...");
    }

    pub fn list_all_employees() {
        println!("Listing all employees...")
    }

    pub fn list_by_department() {
        println!("Listing employees by department...")
    }
}

pub mod prompt {
    pub fn get_input () -> String {
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
        }
    }
}