/**
 * signed/unsigned ints, floats, char, bool, unit
 * type specifiers in literals
 * _ as separators in literals
 * Only explicit conversion using `as` keyword
 * Conversion follow C conventions unless they're undefined; Rust conversions have no undefined behaviour
 *
 * tuples: destructuring, accessors
 * arrays: declaration, type, accessors, length
 * slices: how to get, type, accessors, length
 *
 * Aliasing: only camel case unless disabled with annotation
 */

fn literals() {
     //Signed types
     let n1: i8 = 1;
     let n2: i16 = 2;
     let n3: i32 = 3;
     let n4: i64 = 4;
     let n5: i128 = 5;
     let p: isize = 10;
     assert_eq!(n1, 1);
     assert_eq!(n2, 2);
     assert_eq!(n3, 3);
     assert_eq!(n4, 4);
     assert_eq!(n5, 5);
     assert_eq!(p, 10);

     //Unsigned types
     let un1: i8 = 1;
     let un2: i16 = 2;
     let un3: i32 = 3;
     let un4: i64 = 4;
     let un5: i128 = 5;
     let up: isize = 10;
     assert_eq!(un1, 1);
     assert_eq!(un2, 2);
     assert_eq!(un3, 3);
     assert_eq!(un4, 4);
     assert_eq!(un5, 5);
     assert_eq!(up, 10);

     let f1: f32 = 1.0;
     let f2: f64 = 2.0;
     assert_eq!(f1, 1.0);
     assert_eq!(f2, 2.0);

     let c: char = 'A';
     assert_eq!(c, 'A');

     let b: bool = false;
     assert_eq!(b, false);

     let unit: () = ();
     //Does not implement Display
     assert_eq!(unit, ());

     //Array
     let x = [1];
     //Does not implement Display by default
     assert_eq!(x, [1]);

     //Tuple
     let y = ("cake", 42);
     //Does not implement Display by default
     assert_eq!(y, ("cake", 42));
}

fn literal_specifiers() {
     //speficy tyoe in literal
     let a = 42u32;
     assert_eq!(a, 42);

     //use _ separator anywhere
     let b = 1_000_000_i32;
     assert_eq!(b, 1000000);
     let c = 0.000_001_f32;
     assert_eq!(c, 0.000001);

     //specify literal encoding
     let d = 0b001010010_i64;
     assert_eq!(d, 0b001010010);
}

fn casting() {
     //When casting any value to unsigned type T,
     //T::MAX + 1 is added or substracted
     //until the value fits into the new type
     //1000 - 256 - 256 - 256 = 232
     let x = 1000_i16 as u8;
     assert_eq!(232, x);
     //-1 + 256 = 255
     let y = -1_i16 as u8;
     assert_eq!(255, y);

     //When casting to a signed type, the bitwise result is the same
     //as when casting to the corresponding unsigned type.
     //If the most significant bit is 1, the the value is negative
     //both are represented as 11101000 in binary
     assert_eq!(-24, 1000i32 as i8);
     assert_eq!(232, 1000i32 as u8);
}

fn tuples() {
     //Destructuring
     fn reverse(pair: (i32, bool)) -> (bool, i32) {
          let (integer, boolean) = pair;
          (boolean, integer)
     }

     let pair = reverse((42, true));
     assert_eq!(pair, (true, 42));

     //Accessing pair elements
     assert_eq!(pair.0, true);
     assert_eq!(pair.1, 42);
}

fn arrays_slices() {
     use std::mem;

     //Arrays: listing all values
     let a: [i32; 3] = [0, 1, 2];
     //Arrays: set all value to the same value
     let b: [i32; 20] = [0; 20];

     assert_eq!(a, [0, 1, 2]);
     assert_eq!(b, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
     //Accessing by index
     assert_eq!(a[0], 0);
     //Array size
     assert_eq!(a.len(), 3);
     //Memory consumption of array
     assert_eq!(mem::size_of_val(&a), 12);

     //Array borrowed as slice
     let slice: &[i32] = &a;
     let slice2: &[i32] = &b[0..4];
     //Acessing slice
     assert_eq!(slice[0], 0);
     assert_eq!(slice2.len(), 4);
}

fn aliasing() {
     type NanoSecond = u64;
     #[allow(non_camel_case_types)]
     type u64_t = u64;

     let ns: NanoSecond = 4;
     assert_eq!(ns, 4);

     let x: u64_t = 4;
     assert_eq!(x, 4);
}

pub fn main() {
     literals();
     literal_specifiers();
     casting();
     aliasing();
     tuples();
     arrays_slices();
}
