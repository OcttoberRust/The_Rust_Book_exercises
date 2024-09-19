use std::io;

fn nth_fibonacci_number(nth_number: u32) -> u32 {
   let mut number1 = 1;
   let mut number2 = 1;
   
   let mut count = 0;
   let mut temp = 0;
   
   while count != nth_number - 2 {
       temp = number1 + number2;
       number2 = number1;
       number1 = temp;
       count += 1;
   }

   return temp;
}

fn main() {
    loop {
        let mut input = String::new();

        println!("Please enter an integer greater than 2");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The {input} Fibonacci number is: {}", nth_fibonacci_number(input));
    }    
}
