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
     assert_eq!("", dolly.name);

     dolly.noise();
     dolly.talk();
}

fn derive() {

}

pub fn main() {
     idea();
     derive();
}
