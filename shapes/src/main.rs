

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}


impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

// Static dispatch - A generic function restricted to types that implement types
// Rust will copy paste this function under the hood for each type we pass it 
pub fn print_area_static<T: Shape>(shape: &T) {
    println!("The area of the {} is {:.2}", shape.name(), shape.area());
}

// Dynamic dispatch - A function that takes a Trait object 
// Notice we use &dyn Shape instead of a generic <T:Shape>
pub fn print_area_dynamic(shape: &dyn Shape) {
    println!("(Dynamic) The Area of the {} is {:.2}", shape.name(), shape.area());
}





fn main() {
   let my_circle = Circle { radius: 5.0 };
   let my_rectangle = Rectangle { width: 4.0, height: 6.0};

   // At compile time Rust generate print_area_static_for_circle and print_area_static_for_rectangle

   print_area_static(&my_circle);
   print_area_static(&my_rectangle);


   let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 10.0 }),
    Box::new(Rectangle { width: 2.0, height: 3.0 }),
   ];

   println!("\n--- Iterating through Box<dyn Shape> ---");
   for shape in shapes {
    // Because shape is a Box we can use it like a reference to dyn shape
    // we use .as_ref() to pass it to our dynamic function

    print_area_dynamic(shape.as_ref());
   }
}
