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

/**
 * Closures are structures:
 * 1. Fields refer to captured variables: by & or &mut or by taking ownership
 * 2. closure() desugars to closure.apply()
 *    2.1 If there are owned fields, the signature is "apply(self)" - FnOnce closure's trait - self is muved
 *    2.2 Otherwize, if there are &mut fields, the signature is "apply(&mut self)" - FnMut is closure's trait - self is mutably borrowed
 *    2.3 Otherwise, the signatures is "apply(&self)" - Fn is closure's trait - self is immutably borrowed
 */
fn closure_traits() {
     fn apply_once<F>(f: F)
     where
          F: FnOnce(),
     {
          f();
     }

     fn apply_fnmut<F>(f: &mut F)
     where
          F: FnMut(),
     {
          f();
     }

     fn apply_fn<F>(f: F)
     where
          F: Fn(),
     {
          f();
     }

     //FnOnce can only be passed to apply_once
     {
          let value = String::from("bla");
          let f = || {
               let _x = value;
          };
          apply_once(f);
          // apply_once(f);
     }

     //FnMut can be passed to apply_mut and apply_once
     {
          let mut value = String::from("bla");
          let mut f = || {
               value.push('c');
          };
          apply_fnmut(&mut f);
          //TODO Can't explain this:
          // apply_once(&mut f);
          apply_once(f);

          //No more usage of 'f': it was moved into apply_once
          // apply_fnmut(&mut f); //error
     }

     //Fn can be passed to apply, apply_mut, apply_once
     {
          let string = String::from("asdasd");
          let mut closure = || {
               format!("{}", string);
          };
          apply_fn(closure); //<-- moved here, since signature is "apply_fn<F>(f: F) where F: Fn()"
          apply_fnmut(&mut closure);
          apply_once(closure); //<-- definitely moved here bc it's called inside apply_once as FnOnce
          //TODO: closure has been moved, but these are allowed by the compiler
          apply_fn(closure);
          apply_fnmut(&mut closure);
          apply_once(closure);
          apply_once(closure);
          apply_once(closure);
     }
}

fn functions_as_closures() {
     fn call_me<F: Fn()>(f: F) {
          f();
     }
     fn function() {
          
     }
     call_me(function);
}

fn returning_closures() {
     fn create_fn() -> impl Fn() -> String {
          let x = "cake1";
          move || format!("{}", x)
     }
     let f1 = create_fn();
     let result1 = f1();
     assert_eq!(result1, "cake1");

     fn create_fnmut() -> impl FnMut() -> String {
          let mut x = String::from("cake");
          move || {
               x.push('2');
               format!("{}", x)
          }
     }
     let mut f2 = create_fnmut();
     let result2 = f2();
     assert_eq!(result2, "cake2");

     fn create_fnonce() -> impl FnOnce() -> String {
          let mut x = String::from("cake");
          move || {
               x.push('3');
               let y = x;
               format!("{}", y)
          }
     }
     let f3 = create_fnonce();
     let result3 = f3();
     assert_eq!(result3, "cake3");
}

pub fn main() {
     methods();
     closures();
     closures_capturing();
     forced_closure_capturing_with_move();
     closure_traits();
     functions_as_closures();
     returning_closures();
}
