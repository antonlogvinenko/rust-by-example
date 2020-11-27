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
     assert_eq!(false, dolly.is_naked());
     assert_eq!("Dolly", dolly.name);

     dolly.noise();
     dolly.talk();
}

fn derive() {
     #[derive(PartialEq, PartialOrd)]
     struct Centimiters(f64);

     let x1 = Centimiters(1.0);
     let x2 = Centimiters(2.0);
     assert_eq!(true, x1 < x2);
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

     assert_eq!(FooBar, foobar);
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

pub fn main() {
     idea();
     derive();
     operator_overloading();
     drop();
}
