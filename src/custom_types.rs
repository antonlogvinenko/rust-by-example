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
     let point2 = Point { x:1.0, ..point };
     //Accessing the structure
     println!("Accessing the structure {} {}", point2.x, point2.y);
     //Destructuring the structure
     let Point {x : left, y: right} = point;
     println!("Destructured values: {} {}", left, right);
}

pub fn main() {
     structures();
     println!();
}
