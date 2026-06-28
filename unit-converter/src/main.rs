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

fn miles_to_kilometers(f: f64) -> f64 {
    f * 1.60934
}

fn pounds_to_kilograms(f: f64)-> f64 {
    f * 0.453592
}

fn main() {
    println!("Choose Converter:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Miles to Kilometers");
    println!("3. Pounds to Kilograms");
    println!("Choose an option 1 , 2 or 3");


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

    println!("Select the option: ");
    let mut option = String::new();


    io::stdin()
        .read_line(&mut option)
        .expect("Option not selected properly");

    
    let option: u32 = option.trim().parse().expect("...");


    println!("Enter the number you want to convert: ");
    let mut user_choice = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Not able to read the input");

    
    let user_choice:f64 = user_choice.trim().parse().expect("Please enter a valid number");

    let choice = match option {
        1 => ConversionType::FahrenheitToCelsius,
        2 => ConversionType::MilesToKilometers,
        3 => ConversionType::PoundsToKilograms,
        _ => panic!("Invalid option selected"),
    };

    match choice {
        ConversionType::FahrenheitToCelsius => println!("Fahrenheit to Celsius: {}", fahrenheit_to_celsius(user_choice)),
        ConversionType::MilesToKilometers => println!("Miles to Kilometers: {}", miles_to_kilometers(user_choice)),
        ConversionType::PoundsToKilograms => println!("Pounds to Kilograms: {}", pounds_to_kilograms(user_choice)),
    };


}
