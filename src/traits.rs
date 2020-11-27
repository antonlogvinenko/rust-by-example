fn idea() {
     struct Sheep {
          naked: bool,
          name: &'static str,
     }

     impl Sheep {
          fn is_naked(&self) -> bool {
               self.naked
          }
     }

     trait Animal {
          //Static method signatures: Self refers to the implementor type
          fn new(name: &'static str) -> Self;

          //Non-static method
          fn noise(&self) -> &'static str;
          //Non-static method with default implementation
          fn talk(&self) {
               println!("moooo");
          }
     }

     impl Animal for Sheep {
          fn new(name: &'static str) -> Self {
               Sheep {
                    name: name,
                    naked: false,
               }
          }
          fn noise(&self) -> &'static str {
               "blaaaah"
          }
          fn talk(&self) {
               println!("{}", self.noise());
          }
     }

     let dolly: Sheep = Animal::new("Dolly");
     assert_eq!(dolly.is_naked(), false);
     assert_eq!( dolly.name, "Dolly");

     dolly.noise();
     dolly.talk();
}

fn derive() {
     #[derive(PartialEq, PartialOrd)]
     struct Centimiters(f64);

     let x1 = Centimiters(1.0);
     let x2 = Centimiters(2.0);
     assert_eq!(x1 < x2, true);
}

fn operator_overloading() {
     use std::ops;

     struct Foo;
     struct Bar;

     impl ops::Add<Bar> for Foo {
          type Output = FooBar;

          fn add(self, _rhs: Bar) -> FooBar {
               FooBar
          }
     }

     #[derive(PartialEq, Debug)]
     struct FooBar;

     let foo = Foo;
     let foobar = foo + Bar;

     assert_eq!(foobar, FooBar);
}

fn drop() {
     struct Droppable {
          name: &'static str,
     };

     impl Drop for Droppable {
          fn drop(&mut self) {
               println!("> Dropping {}", self.name);
          }
     }

     let _d = Droppable { name: "asd" };
}

fn iterators() {
     struct Fibonacci {
          curr: u32,
          next: u32,
     }

     impl Iterator for Fibonacci {
          type Item = u32;

          fn next(&mut self) -> Option<u32> {
               let new_next = self.curr + self.next;
               self.curr = self.next;
               self.next = new_next;
               Some(self.curr)
          }
     }

     //using interface directly
     let mut sequence = 0..3;
     assert_eq!(sequence.next().unwrap(), 0);
     assert_eq!(sequence.next().unwrap(), 1);

     //using interface indirectly
     let mut x = 0;
     for _ in 0..3 {
          x += 1;
     }
     assert_eq!(x, 3);

     //using manually defined interface directly
     let fib = Fibonacci { curr: 0, next: 1 };
     let p = fib.skip(3).take(1).next().unwrap();
     assert_eq!(p, 3);

     //using manually defined interface indirectly
     for x in (Fibonacci { curr: 0, next: 1 }).take(3) {
          println!("{}", x);
     }

     let array = [1u32, 3, 3, 7];
     let mut y = 0;
     for i in array.iter().skip(1).take(2) {
          y += i;
     }
     assert_eq!(y, 6);
}

pub fn main() {
     idea();
     derive();
     operator_overloading();
     drop();
     iterators();
}
