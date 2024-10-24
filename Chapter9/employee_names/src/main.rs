use std::collections::HashMap;
use std::io;

fn main() {

    let mut input = String::new();
    let mut map: HashMap::<String, String> = HashMap::new();

    println!("Please enter the department name and employee name in this format: 
             \'Add *Name* to *Department*\'"); 
    io::stdin().read_line(&mut input);

    let employee = parse_name(&input); 
    map.insert(employee.name, employee.department);
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
    let mut parsed_string = input.split_whitespace();

    let mut _name = String::new();
    let mut _department = String::new();

    if let (Some(second), Some(fourth)) = (parsed_string.nth(1), parsed_string.nth(2)) {

        _name = String::from(second);
        _department = String::from(fourth);
    };

    Employee {
        name: _name,
        department: _department,
    }
}
