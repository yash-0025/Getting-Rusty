use std::rc::Rc;
use std::cell::RefCell;



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


// fn main() {
//     println!("Calculator started!");

//     // Building the 3.0 * 2.0 part using Box::new() to put them on the heap
//     let three = Box::new(Expr::Number(3.0));
//     let two = Box::new(Expr::Number(2.0));
//     let multiplication = Box::new(Expr::Mul(three, two));

//     // 2. Build the 5.0 part
//     let five = Box::new(Expr::Number(5.0));

//     let math_tree = Expr::Add(five, multiplication);

//     let result = math_tree.eval();

//     println!("5.0 + (3.0 * 2.0) = {}", result);


//     // We want to calculate (5.0+3.0) * (5.0+3.0)
//     // Instead of building (5.0+3.0) twice we build it ONCE and share it 
//     let five = Expr::Number(5.0);
//     let three = Expr::Number(3.0);

//     // We wrap it in an Rc so it can have multiple owners
//     let shared_addition = Rc::new(Expr::Add(Box::new(five), Box::new(three)));

//     println!("Right now, the counter is at: {}", Rc::strong_count(&shared_addition));

//     // We use .clone() on the RC
//     // This Does not copy the data it just increments the counter to 2.
//     let left_side = Rc::clone(&shared_addition);
//     let right_side = Rc::clone(&shared_addition);

//     println!("Now the counter is at : {}", Rc::strong_count(&shared_addition));

//     // Note - We can't put Rc inside our current Mul because our enum expects Box
//     // But this demonstrates exactly how Rc tracks ownership without copying data


//     // Create a shared mutable number
//     let shared_number = Rc::new(RefCell::new(10));

//     // Create a second owner increments TC counter to 2
//     let owner_two = Rc::clone(&shared_number);

//     // Mutate the data through owner_two
//     {
//         // We ue .borrow_mut() to ask the Security guard for write access
//         // We add * to derefernce it so we can change the actual integer
//         let mut mutable_reference = owner_two.borrow_mut();
//         *mutable_reference += 5;

//         // As soon as this block ends mutable_reference is dropped and the Security guard locks the glass case again 

//     }

//     // Read the data through the original owner
//     // We use .borrow() to ask the Security guard for read access
//     println!("The shared number is now: {}", shared_number.borrow());

// }


// // We are creating a number 10 sharing it between two owners changing it via the second owner and reading the changes value from the first owner
// // We wrap the number in RefCell for mutability and then wrap that in Rc for shared ownership. We use .borrow_mut() to write and .borrow() to read
// // Normally Rust forbids this if we have two owners the data must be completely immutable by combining Rc and RefCell, we bypass the compile time checks and safely achieveshared mutable state at runtime


#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}


pub struct Lexer {
    chars: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // Skip spaces : keep moving the position forward as long as we see empty spaces
        while self.position < self.chars.len() && self.chars[self.position].is_whitespace() {
            self.position += 1;
        }
        
        // Check if we reached the end of the string
        if self.position >= self.chars.len() {
            return None;
        }

        // Look at the current character
        let current_char = self.chars[self.position];

        // handle single character operators
        match current_char {
            '+' => {
                self.position += 1;
                return Some(Token::Plus);
            }
            '-' => {
                self.position += 1;
                return Some(Token::Minus);
            }
            '*' => {
                self.position += 1;
                return Some(Token::Multiply);
            }
            '/' => {
                self.position += 1;
                return Some(Token::Divide);
            }
            '(' => {
                self.position += 1;
                return Some(Token::LParen);
            }
            ')' => {
                self.position += 1;
                return Some(Token::RParen);
            }
            _ => {} // Its not a simple operator so we fall throgh to the number logic below
        }
        
        // Handle numbers
        if current_char.is_digit(10) {
            let mut number_str = String::new();

            // Keep reading characters as long as they are digits or a decimal point
            // FIXED: Using == instead of =
            while self.position < self.chars.len() && 
                 (self.chars[self.position].is_digit(10) || self.chars[self.position] == '.') {
                
                // Add the character to our string
                number_str.push(self.chars[self.position]);
                self.position += 1;
            }

            // convert the string into a f64 
            // .unwrap() will panic if it's an invalid float like 42.5.5
            let parsed_number: f64 = number_str.parse().unwrap();
            return Some(Token::Number(parsed_number));
        }
        
        // If its anything else just panic for now
        panic!("Unknown character found {}", current_char);
    }
}

pub struct Parser {
    // We take the list of tokens from the lexer
    tokens: Vec<Token>,
    // We track which token we are currently looking at
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0}
    }

    // A helper to safely look at the current token without crashing
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // Level 1 Parse a factor numbers and parantheses
    pub fn parse_factor(&mut self) -> Expr {
        let token = self.current_token().expect("Unexpected end of tokens");

        match token {
            // If it's a number , we return the number ASTnode and move forward
            Token::Number(n) => {
                let value = *n; // copy f64 out of the reference
                self.position += 1;
                return Expr::Number(value);
            }
            // If it's a left Paranthesis we must parse an entire inner expression
            Token::LParen => {
                self.position += 1; // Skip the (
                // WE recursively call parse_expression 

                let inner_expr = self.parse_expression();

                if self.current_token() == Some(&Token::RParen) {
                    self.position += 1;
                } else {
                    panic!("Missing closing paranthesis!");
                }

                return inner_expr;
            }
            _ => panic!("Exprected a number or '("),
        }
    }

    pub fn parse_term(&mut self) -> Expr {
        // first get the left side which is just a factor or parentheses
        let mut left_side = self.parse_factor();
        // as long as the next token is a * or /, we keep building the tree
        while let Some(token) = self.current_token() {
            match token {
                Token::Multiply => {
                    self.position += 1;
                    let right_side = self.parse_factor(); // get the right side

                    left_side = Expr::Mul(Box::new(left_side), Box::new(right_side));
                }

                Token::Divide => {
                    self.position += 1;
                    let right_side = self.parse_factor();
                    left_side = Expr::Mul(Box::new(left_side), Box::new(right_side));
                }
                _ => break // If it's not * or / we are done with this term
            }
        }
        return left_side;
    }

    pub fn parse_expression(&mut self) -> Expr {
        // first get the left side which is a term
        let mut left_side = self.parse_term();

        // As long as the next token is a + or -, we keep building the tree
        while let Some(token) = self.current_token() {
            match token {
                Token::Plus => {
                    self.position += 1;
                    let right_side = self.parse_term();
                    left_side = Expr::Add(Box::new(left_side), Box::new(right_side));
                }

                Token::Minus => {
                    self.position += 1;
                    let right_side = self.parse_term();
                    left_side = Expr::Sub(Box::new(left_side), Box::new(right_side));
                }
                _ => break,
            }
        }

        return left_side;
    }
}

fn main() {

    let input = "(5.0 + 3.0) * 2.0";
    println!("Evaluating: {}", input);

    // Lexing :: convert string to Tokens
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();


    while let Some(token) = lexer.next_token() {
        // println!("{:?}", token);
        tokens.push(token);
    }

    // Convert Tokens to AST
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expression();

    // Evaluation :Calculate the result using our invisible buttler dered
    let result = ast.eval();

    println!("Result: {}", result);
}



// The parser struct holds our Vec<Token> amd an index. parse_factor looks at the current token. If it's a number it builds and Expr::Number and moves forward if it's a ( it recursively parses everything inside the parantheses and then make sure thre is a ) at the end
// How it works - We use the exact same array indexing logic as the Lexer self.position we use a match statement on the enum Notice the recusion if  we hit a paranthesis we call parse_expression() to handle the math inside
// By separating Factors numbers/parens from Expressions addition we naturally enforc the Order of Operation PEMDAS parentheses and numbers are processed first


// What we did lastly ? 
// parse_term specifically looks for * or / . parse expression specifically looks for + or -
// How it works ? = Look closely at the hierarchy parse_expressoin call parse_term. parse_term calls parse_factor
// Why ? => By layering these function calls we naturally build PEMDAS into the structure of our code. The compiler is forced to calculate parse_factor parentheses before parse_term multiplication and parse_term before parse_expression addition


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function so we don't have to repeat the Lexer loop in every test
    fn parse_string(input: &str) -> Expr {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        let mut parser = Parser::new(tokens);
        parser.parse_expression()
    }

    #[test]
    fn test_valid_math() {
        let expr = parse_string("5.0 + 3.0 * 2.0");
        assert_eq!(expr.eval(), 11.0);
    }

    #[test]
    #[should_panic(expected = "Exprected a number")]
    fn test_bad_syntax_panics() {
        // this is invalid syntax . The parser should panic
        parse_string("5.0 + * 3.0");
    }
}

// What it does - The first test checks normal math 5.0 + 3.0 * 2.0 = 11 .
// The second test feeds garbage into the parser and checks if it panics
// How it works? => #[should_panic] tells the test runner. If this function doesnot crash fail the test.
// By adding expected = ... we tell rust to only pass the test if the panic mesage matches our string exactly
// why ? => We dont want a crash happening deep in production code because of an edge case we missed. Writing panic test guarantees our code fails exactly when and how we designed it to.
