use std::fs;

// fn main() {
 
//     let x = 5;
//     let y = x;

//     println!("x is {} and y is {}", x,y);

//     // So here this above statement will work perfectly fine because x is an i32 a simple integer on stack . It is so tiny and simple that Rust automatically copies it . When we say let y=x rust just writes another sticky note . Both x and y exist independently

//     // But s1 is a string on the heap . It's huge . Rust refuses to automatically copy huge things because it would make our program slow. So instead of copying the book it MOVES the library card from s1 to s2. s1 is now empty.

//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 is {} and s2 is {}", s1,s2);
// }

// Borrowing
// fn take_ownership(s: String) {
//     println!("The function owns: {}", s);
// }

// fn borrow_string(s: &String) {
//     println!("The function is borrowing: {}", s);
// }

// fn main() {

//     let my_name = String::from("Yash");

//     borrow_string(&my_name);

//     println!("I still own: {}", my_name);

//     take_ownership(my_name);

//     println!("Can i print it? {}", my_name)
// }


// Mutable Borrowing

// fn append_surname(name: &mut String) {
//     name.push_str(" Patel");
// }

// fn main() {

//   let mut my_name = String::from("Yash");

//   append_surname(&mut my_name);

//   println!("My full name is : {}", my_name);
// }

fn main() {

    // REad the current directory ./
    //  expect() will crash the program if the folder doesn't exist 
    let folders = fs::read_dir("./").expect("Failed to read directory");

    // Looping through every item in the folder
    for item in folders {
        // Unpack the item because reading a specific file can also fail!
        let file = item.expect("Failed to read file");

        let metadata = file.metadata().expect("Failed to get metadata");

        if metadata.is_file() {
            let size = metadata.len(); // returns the size in bytes as u64
        
            println!("File: {:?} | Size; {} bytes", file.file_name(), size);

        }
    }
}