
fn main() {
    let presents_of_christmas: [&str; 12] = 
        ["a partridge in a pear tree",
         "two turtle doves",
         "three french hens",
         "four calling birds",
         "five gold rings",
         "six geese a-laying",
         "seven swans swimming",
         "eight maids a-milking",
         "nine ladies dancing",
         "ten lords a leaping",
         "eleven pipers piping",
         "twelve drummers drumming"];

    let mut day = 0;
    let mut verse_count = 0;

    //You should be able to bring down the time complexity
    //with a mutable compound data type
    while(day != 12) {
        
        println!("On the {} day of Christmas my true love gave to me: ", day+1);

        while(verse_count <= day) {

           println!("{}", presents_of_christmas[day - verse_count]);

           verse_count += 1;
        }

        day += 1;
        verse_count = 0;
    }
}
