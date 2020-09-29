fn methods() {
     struct Pair(i32, i32);
     impl Pair {
          fn sum(&self) -> i32 {
               self.0 + self.1
          }

          fn inc(&mut self) {
               self.0 += 1;
               self.1 += 1;
          }

          fn destroy(self) {}
     }

     let pair = Pair(1, 2);
     //self borrowed via immutable reference
     let sum = pair.sum();
     assert_eq!(sum, 3);
     pair.destroy();
     //self is moved and dropped, pair is not available further in scope
     // pair.sum();

     let mut pair2 = Pair(3, 4);
     //mutably borrow pair2 to modify fields
     pair2.inc();
     //self borrowed via immutable reference
     let sum2 = pair2.sum();
     assert_eq!(sum2, 9);
     //self is moved and dropped, pair is not available further in scope
     pair2.destroy();
     // pair.sum();
}

fn closures() {
     let x = 10;
     let annotated = |i: i32| -> i32 { i + x };
     let inferred = |i| i + x;

     assert_eq!(annotated(5), 15);
     assert_eq!(inferred(6), 16);
}

fn closures_capturing() {
     //Borrowing captured variables by immutable reference
     {
          let color = String::from("green");
          //--> Sart holding immutable reference to color
          let print = || println!("Color is {}", color);

          //Can only borrow immutable while immutable the reference is held
          let _color_ref = &color;
          // let color_mut_ref = &mut color;
          // let color_move = color;

          //Using 'color' that was borrowed by immutable reference
          print();
          //<-- End holding immutable reference to 'color'
     }

     //Borrowing captured variables by mutable reference
     {
          let mut count = 0;
          //--> Start holding mutable reference to color
          let mut inc = || {
               count += 1;
          };
          // let count2 = &count;
          // let count3 = &mut count;
          // let count4 = count;

          //Using 'count' that was borrowed by mutable reference
          inc();
          //<-- End holding mutable reference to 'count'
     }

     //Moving captured variables
     {
          let string = String::from("cake");
          //->> Start owning 'string'
          let consume = || {
               let _p = string;
          };

          //already moved 'string'
          // let string2 = string;
          //already moved 'string'
          // let string3 = &string;
          //already moved 'string'
          // let string4 = &mut string;
          consume();

          //Can't call twice: already moved 'string'
          // consume();
     }
}

fn forced_closure_capturing_with_move() {
     let color = String::from("green");
     //--> Sart owning color
     let print = move || {
          let _owning = color;
          // mem::drop(color);
     };
     // let _color_ref = &color;
     // let color_mut_ref = &mut color;
     // let color_move = color;

     //Using 'color' that was moved in
     print();
     //<-- End holding immutable reference to 'color'
     // print();
}

pub fn main() {
     methods();
     closures();
     closures_capturing();
     forced_closure_capturing_with_move();
}
