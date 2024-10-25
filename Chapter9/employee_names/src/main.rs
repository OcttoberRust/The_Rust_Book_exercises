use std::collections::HashMap;
use std::io;

fn main() {

    let mut input = String::new();
    let mut map: HashMap::<String, String> = HashMap::new();

    println!("Please enter the department name and employee name in this format: 
             \'Add *Name* to *Department*\'"); 
    io::stdin().read_line(&mut input);
    println!("{:?}", input);
    let employee = parse_name(&input); 
    println!("{:?}", employee.name);
    map.insert(employee.name, employee.department);
    println!("{:?}", map);
    //ask the user if they would like to enter a employee/department or exit the program
    //if condition for entering employee 
    //send input string to function to parse input and return employee name and department name
    //statement to insert department name (key)
    //statement to add employee by department name (might be able to consolidate)
    //else condition to exit program

}

struct Employee {
    name: String,
    department: String
}

fn parse_name(input: &String) -> Employee {
    let mut parsed_string: Vec<&str> = input.split_whitespace().collect();

    //Good practice would use option handling here
    Employee {
        name: parsed_string.get(1).expect("unknown").to_string(),
        department: parsed_string.get(3).expect("unknown").to_string(),
    }
}
