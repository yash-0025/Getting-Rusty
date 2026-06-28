use std::io;

// enum Category {
//     Distance,
//     Temperature,
// } 

enum ConversionType {
    FahrenheitToCelsius,
    MilesToKilometers,
    PoundsToKilograms,
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0 ) * (5.0 / 9.0)
}


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

    // println!("Please input your number: ");

    // let mut user_input = String::new();


    // io::stdin()
    //     .read_line(&mut user_input)
    //     .expect("Failed to read line");

    // println!("You entered: {}", user_input);
    

    // let user_input: f64 = user_input.trim().parse().expect("Please type a real number!");

    // println!("Double your number is {}", user_input * 2.0);

// Eg - Enums 

    // let choice = Category::Distance;

    // match choice {
    //     Category::Distance => println!("You choose distance!"),
    //     Category::Temperature => println!("You choose temperature!"),
    // }

//  Main program

    println!("Enter the number you want to convert: ");
    let mut user_choice = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Not able to read the input");

    
    let user_choice:f64 = user_choice.trim().parse().expect("Please enter a valid number");

    let choice = ConversionType::FahrenheitToCelsius;

    match choice {
        ConversionType::FahrenheitToCelsius => println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(user_choice)),
        ConversionType::MilesToKilometers => println!("You choose Miles to Kilometers"),
        ConversionType::PoundsToKilograms => println!("You choose Pounds to Kilograms"),
    }

}
