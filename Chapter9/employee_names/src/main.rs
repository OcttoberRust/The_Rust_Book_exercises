use std::collections::HashMap;
use std::io;

fn main() {

    let mut input = String::new();
    let mut map: HashMap::<String, String> = HashMap::new();

    println!("
              Enter 1 to view all employees and their departments.
              Enter 2 to find an employee by department. 
              Enter 3 to enter an employee into a department.
              Enter 4 to exit the program");

    match input.trim() {
        "1" => {
            input.clear();
            println!("{:?}", map);
        },
        "2" => {
            input.clear(); 
            find_employee(&map);
        },
        "3" => println!("success 3"),
        "4" => println!("success 4"),
        (_) => println!("Please enter a valid input."),
    }

    input.clear();

    println!("Please enter the department name and employee name in this format: 
             \'Add *Name* to *Department*\'"); 
    io::stdin().read_line(&mut input);
    println!("{:?}", input);
    let employee = parse_name(&input); 
    println!("{:?}", employee.name);
    map.insert(employee.department, employee.name);
    println!("{:?}", map);

    
    
    //ask the user if they would like to enter a employee/department or exit the program
    //if condition for entering employee 
    //send input string to function to parse input and return employee name and department name
    //statement to insert department name (key)
    //statement to add employee by department name (might be able to consolidate)
    //else condition to exit program

}

struct Employee {
    department: String,
    name: String
}


fn parse_name(input: &String) -> Employee {
    let mut parsed_string: Vec<&str> = input.split_whitespace().collect();

    //Good practice would use option handling here
    Employee {
        department: parsed_string.get(3).expect("unknown").to_string(),
        name: parsed_string.get(1).expect("unknown").to_string(),
    }
}

fn find_employee(map: &HashMap<String, String>) {

    let mut input_employee_name = String::new(); 

    println!("Please enter the employee's department, followed by their name. Format: Department EName");

    io::stdin().read_line(&mut input_employee_name).expect("Failed to read line.");


    let mut parsed_string: Vec<&str> = input_employee_name.split_whitespace().collect();

    //Good practice would use option handling here
    let employee = Employee {
        department: parsed_string.get(0).expect("unknown").to_string(),
        name: parsed_string.get(1).expect("unknown").to_string(),
    };

    match map.get(&employee.department) {
        None => println!("No key found!"),
        Some(v) => {
            println!("Printing vec: {:?}", v);
        },
    };

}
