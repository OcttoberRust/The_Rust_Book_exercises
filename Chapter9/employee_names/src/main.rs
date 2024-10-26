use std::collections::HashMap;
use std::io;

fn main() {

    let mut input = String::new();
    let mut map: HashMap::<String, Vec<String>> = HashMap::new();

    loop {
        println!("
        Enter 1 to view all employees and their departments.
        Enter 2 to find an employee by department. 
        Enter 3 to enter an employee into a department.
        Enter 4 to exit the program.\n");

        io::stdin().read_line(&mut input).expect("failed to read line"); 
        match input.trim() {
            "1" => {
                input.clear();
                println!("{:?}", map);
            },
            "2" => {
                input.clear(); 
                find_employee(&map);
            },
            "3" => {
                input.clear();
                insert_employee(&mut map);
            },
            "4" => {
                input.clear();
                break;
            },
            (_) => {
                input.clear();
                println!("Please enter a valid input");
            }
        }
    }
}

struct Employee {
    department: String,
    name: String
}

fn find_employee(map: &HashMap<String, Vec<String>>) {

    let mut input_find_employee = String::new(); 

    println!("Please enter the employee's department, followed by their name. Format: Department EName");

    io::stdin().read_line(&mut input_find_employee).expect("Failed to read line.");

    let mut parsed_string: Vec<&str> = input_find_employee.split_whitespace().collect();

    //Good practice would use option handling here
    let employee = Employee {
        department: parsed_string.get(0).expect("unknown").to_string(),
        name: parsed_string.get(1).expect("unknown").to_string(),
    };

    match map.get_key_value(&employee.department) {
        None => println!("No key found!"),
        Some((key, value)) => {
            if let Some(found_employee_name) = value.iter().find(|&name| name == &employee.name) {
                println!("{} is in {} department!", found_employee_name, key);
            }
        },
    };

}

fn insert_employee(map: &mut HashMap<String, Vec<String>>) {

    let mut input_insert_employee = String::new();

    println!("Please enter the department name and employee name in this format: 
             \'Add *Name* to *Department*\'"); 

    io::stdin().read_line(&mut input_insert_employee);

    let mut parsed_string: Vec<&str> = input_insert_employee.split_whitespace().collect();

    //Good practice would use option handling here
    let employee = Employee {
        department: parsed_string.get(3).expect("unknown").to_string(),
        name: parsed_string.get(1).expect("unknown").to_string(),
    };

    match map.get_mut(&employee.department) {
        None => {
            //insert
            let mut name_vec: Vec<String> = Vec::<String>::new();
            name_vec.push(employee.name);
            map.insert(employee.department, name_vec);
        }
        Some(returned_vec) => {
            returned_vec.push(employee.name);
        }
    };

    println!("{:?}", map);
}
