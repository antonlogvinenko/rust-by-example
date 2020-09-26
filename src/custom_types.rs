/**
 * Structures: no args, unnamed args, named args
 * Constructing all three
 * Descrutcuring all three
 * Accessing all three
 * 
 * Enums, +structures
 * Pattern matching
 * Aliases
 * 
 * C-like structures
 */


fn structures() {
     #[derive(Debug)]
     struct Unit;

     #[derive(Debug)]
     struct Pair(i32, f32);

     #[derive(Debug)]
     struct Point {
          x: f32,
          y: f32,
     }

     println!("Trivial structure: {:?}", Unit);

     let pair = Pair(1, 2.0);
     println!("Pair: {:?}", pair);
     //Accessing the structure
     println!("Pair cdr: {} {}", pair.0, pair.1);
     //Destructuring the structure
     let Pair(car, cdr) = pair;
     println!("Destructured parts of Pair: {} {}", car, cdr);

     let point = Point { x: 1.0, y: 2f32 };
     //Struct update syntax
     let point2 = Point { x: 1.0, ..point };
     //Accessing the structure
     println!("Accessing the structure {} {}", point2.x, point2.y);
     //Destructuring the structure
     let Point { x: left, y: right } = point;
     println!("Destructured values: {} {}", left, right);
}

fn enums() {
     #[derive(Debug)]
     enum WebEvent {
          PageLoad,
          PageUnload,
          KeyPress(char),
          Paste(String),
          Click { x: i64, y: i64 },
     }

     fn inspect(event: WebEvent) {
          match event {
               WebEvent::PageLoad => println!("page loaded"),
               WebEvent::PageUnload => println!("page unloaded"),
               WebEvent::KeyPress(c) => println!("key {} pressed", c),
               WebEvent::Paste(paste) => println!("pasted string {}", paste),
               WebEvent::Click { x, y } => {
                    println!("Clicked at ({}, {})", x, y);
               }
          }
     }

     inspect(WebEvent::PageLoad);
     inspect(WebEvent::PageUnload);
     inspect(WebEvent::KeyPress('k'));
     inspect(WebEvent::Paste("text".to_owned()));
     inspect(WebEvent::Click { x: 32, y: 4 })
}

fn enum_aliases() {
     #[derive(Debug)]
     enum AdfsdfsdfLsdfdsffgsdfsfasafJfsdfsdfFactoryBuilder {
          Add,
          Substract,
     }
     type FactoryBuilder = AdfsdfsdfLsdfdsffgsdfsfasafJfsdfsdfFactoryBuilder;
     let x = FactoryBuilder::Add;
     println!("Short type name {:?}, {:?}", x, FactoryBuilder::Substract);
}

fn c_like_enums() {
     #[derive(Debug)]
     enum Color {
          Red = 0xff0000,
          Green = 0x00ff00,
          Blue = 0x0000ff,
     }

     println!("C like enum: {:?} {:?} {:?}", Color::Red, Color::Green, Color::Blue);
}

fn constants() {
     const THRESHOLD: i32 = 42;
     static LANGUAGE: &str = "Rust";

     println!("const {}", THRESHOLD);
     println!("static {}", LANGUAGE);
}

pub fn main() {
     structures();
     println!();

     enums();
     println!();

     enum_aliases();
     println!();

     c_like_enums();
     println!();
     
     constants();
     println!();
}
