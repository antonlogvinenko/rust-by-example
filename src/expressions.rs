pub fn main() {
     //Rust program is made of statements
     //Statements: variable binding, expression with ';'
     
     //Blocks are expressions
     //Last expression of the block is the result of the block
     //if the last expression ends with ';' the result is ()
     let x = 3;
     let y = {
          let h = 5;
          h * x + 10
     };
     assert_eq!(y, 25);

     //';' separates expressions
     //variable bindings is a statement and must be ended with ';'
}