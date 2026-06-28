use std::io;

 

fn main() {

    // let mut score = 50;
    // println!("The Score is {}", score);

    // score = 100;
    // println!("The new Score is {}", score);

    // let name = 1234;
    // println!("The name is {}", name);

    // let name = "Yash";
    // println!("The new name is {}", name);

    // let spaces = "   ";
    // println!("Spaces string : {}", spaces);

    // let spaces = spaces.len();
    // println!("Number of spaces: {}", spaces );

    println!("Please input your number: ");

    let mut user_input = String::new();


    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("You entered: {}", user_input);
    

    let user_input: f64 = user_input.trim().parse().expect("Please type a real number!");

    println!("Double your number is {}", user_input * 2.0);


}
