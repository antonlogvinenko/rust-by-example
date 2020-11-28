
fn panic() {
     //use env var RUST_BACKTRACE=0|1|full
     //panic!("bla");
}

fn option() {
     //Option: Some(T) | None
     let x = Some(42);
     let y: Option<i32> = None;

     //unsafe: .unwrap => panic!
     // y.unwrap();
     assert_eq!(x.unwrap(), 42);

     //safe: pattern matching or ? operator
     fn increase1(x: Option<i32>) -> Option<i32> {
          let value = x?;
          Some(value + 1)
     }
     fn increase2(x: Option<i32>) -> Option<i32> {
          match x {
               Some(value) => Some(value + 1),
               None => None,
          }
     }
     assert_eq!(increase1(x), Some(43));
     assert_eq!(increase1(y), None);
     assert_eq!(increase2(x), Some(43));
     assert_eq!(increase2(y), None);

     //combining with map and map_then
     let x2 = x.map(|v| v + 1);
     let x3 = x.and_then(|v| Some(v));
     assert_eq!(x2, Some(43));
     assert_eq!(x3, Some(42));
}

pub fn main() {
     panic();
     option();

}