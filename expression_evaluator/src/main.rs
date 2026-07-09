use std::rc::Rc;


pub enum Expr {
    // A simple number like 5.0
    Number(f64),
    // An addition of two expressions like 5.0 +3.0
    // The treasure maps Box point to the Heap where the inner Expr lives
    // Add(Expr, Expr),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    
}

impl Expr {
    // We pass &self so we can read the tree without destroying it 
    pub fn eval(&self) -> f64 {
        match self {
            // It it's just a number , dereference the pointer and return the f64
            Expr::Number(n) => *n,

            // Deep Pattern Matching
            // We recursively call `.eval()` on the left side and the right side
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Sub(left, right) => left.eval() - right.eval(),
            Expr::Mul(left, right) => left.eval() * right.eval(),
            Expr::Div(left, right) => left.eval() / right.eval(),
            
        }
    }
}


fn main() {
    println!("Calculator started!");

    // Building the 3.0 * 2.0 part using Box::new() to put them on the heap
    let three = Box::new(Expr::Number(3.0));
    let two = Box::new(Expr::Number(2.0));
    let multiplication = Box::new(Expr::Mul(three, two));

    // 2. Build the 5.0 part
    let five = Box::new(Expr::Number(5.0));

    let math_tree = Expr::Add(five, multiplication);

    let result = math_tree.eval();

    println!("5.0 + (3.0 * 2.0) = {}", result);


    // We want to calculate (5.0+3.0) * (5.0+3.0)
    // Instead of building (5.0+3.0) twice we build it ONCE and share it 
    let five = Expr::Number(5.0);
    let three = Expr::Number(3.0);

    // We wrap it in an Rc so it can have multiple owners
    let shared_addition = Rc::new(Expr::Add(Box::new(five), Box::new(three)));

    println!("Right now, the counter is at: {}", Rc::strong_count(&shared_addition));

    // We use .clone() on the RC
    // This Does not copy the data it just increments the counter to 2.
    let left_side = Rc::clone(&shared_addition);
    let right_side = Rc::clone(&shared_addition);

    println!("Now the counter is at : {}", Rc::strong_count(&shared_addition));

    // Note - We can't put Rc inside our current Mul because our enum expects Box
    // But this demonstrates exactly how Rc tracks ownership without copying data
}
