use std::io;

fn nth_fibonacci_number(nth_number: u32) -> u32 {
   let mut number1 = 1;
   let mut number2 = 1;
   
   let mut count = 0;
   let mut temp = number1 + number2;
   
   while count != nth_number {
       temp = number2 + number1;
       number2 = temp;
       number1 = number2;
       count += 1;
   }

   return temp;
}

fn main() {
    println!("Hello, world!");
    //todo: add call, validate fn, ensure correctness, add user input
}
