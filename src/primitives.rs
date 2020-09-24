
fn literals() {
     //Signed types
     let n1: i8 = 1;
     let n2: i16 = 2;
     let n3: i32 = 3;
     let n4: i64 = 4;
     let n5: i128 = 5;
     let p: isize = 10;
     println!("signed integers: {} {} {} {} {} {}", n1, n2, n3, n4, n5, p);
     
     //Unsigned types
     let un1: i8 = 1;
     let un2: i16 = 2;
     let un3: i32 = 3;
     let un4: i64 = 4;
     let un5: i128 = 5;
     let up: isize = 10;
     println!("unsigned integers: {} {} {} {} {} {}", un1, un2, un3, un4, un5, up);

     let f1: f32 = 1.0;
     let f2: f64 = 2.0;
     println!("floats: {} {}", f1, f2);

     let c: char = 'A';
     println!("char {}", c);

     let b: bool = false;
     println!("boolean: {}", b);

     let unit: () = ();
     //Does not implement Display
     println!("unit: {:?}", unit);

     //Array
     let x = [1];
     //Does not implement Display by default
     println!("Array {:?}", x);

     //Tuple
     let y = ("cake", 42);
     //Does not implement Display by default
     println!("Tuple {:?}", y);
}

fn literal_specifiers() {
     //speficy tyoe in literal
     let a = 42u32;
     //use _ separator anywhere
     let b = 1_000_000_i32;
     let c = 0.000_001_f32;
     //specify literal encoding
     let d = 0b001010010_i64;
     println!("Literals with specifiers: {} {} {} {}", a, b, c, d);
}

fn tuples() {
     //Destructuring
     fn reverse(pair: (i32, bool)) -> (bool, i32) {
          let (integer, boolean) = pair;
          (boolean, integer)
     }

     let pair = reverse((42, true));
     println!("Pair: {:?}", pair);

     //Accessing pair elements
     println!("Pair element 0 {}", pair.0);
}

fn arrays_slices() {
     use std::mem;

     //Arrays: listing all values
     let a: [i32; 3] = [0, 1, 2];
     //Arrays: set all value to the same value
     let b: [i32; 20] = [0; 20];

     println!("These are arrays {:?} {:?}", a, b);
     println!("Accessing array by index: {}", a[0]);
     println!("Reading array size: {}", a.len());
     println!("Analyze array memory size: {}", mem::size_of_val(&a));

     //Array borrowed as slice
     let slice: &[i32] = &a;
     let slice2: &[i32] = &a[0..2];
     //Acessing slice
     println!("Accessing slice {}", slice[0]);
     println!("Slice length {}", slice2.len());
}

pub fn main() {
     literals();
     println!();

     literal_specifiers();
     println!();

     tuples();
     println!();

     arrays_slices();
     println!();
}