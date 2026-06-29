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

fn take_ownership(s: String) {
    println!("The function owns: {}", s);
}

fn borrow_string(s: &String) {
    println!("The function is borrowing: {}", s);
}

fn main() {

    let my_name = String::from("Yash");

    borrow_string(&my_name);

    println!("I still own: {}", my_name);

    take_ownership(my_name);

    println!("Can i print it? {}", my_name)
}