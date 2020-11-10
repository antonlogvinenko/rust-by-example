use std::fmt::Debug;

/**
 * Variables own resources.
 * When vairables goes out of scope, resource is destroyed.
 * Destructor is provided via Drop trait.
 *
 * Avoiding memory leaks
 */
fn raii() {
     let box1 = Box::new(42);

     {
          let box2 = Box::new(32);
          assert_eq!(box2.as_ref(), &32);
          //box2 is destroyed here
     }

     //box1 is destroyed here
     assert_eq!(box1.as_ref(), &42);

     struct ToDrop;

     impl Drop for ToDrop {
          fn drop(&mut self) {
               println!("Value dropped!")
          }
     }

     let _dropped = ToDrop;
}

/**
 * Since variables are in charge of freeing resources,
 * resources can only have one owner.
 * Avoiding freeing more than once.
 *
 * Previous owner can't access resource.
 * Avoiding dangling pointers.
 */
fn ownership_and_moves() {
     fn destroy(_: Box<i32>) {}
     let a = Box::new(5i32);
     assert_eq!(a.as_ref(), &5);

     let b = a;
     //Can do
     assert_eq!(b.as_ref(), &5);
     //Can't do
     //assert_eq!(a.as_ref(), &5);
     destroy(b);
     //Can't do
     // assert_eq!(b.as_ref(), &5);

     let immutable_box = Box::new(5);
     assert_eq!(immutable_box.as_ref(), &5);
     let mut mutable_box = immutable_box;
     assert_eq!(mutable_box.as_ref(), &5);

     *mutable_box = 1;
     assert_eq!(mutable_box.as_ref(), &1);
}

fn moving_vs_borrowing() {
     fn move_i32(_: Box<i32>) {}
     fn borrow_i32(_: &Box<i32>) {}

     let x = Box::new(42);
     borrow_i32(&x);

     {
          let y = &x;
          //can't move while borrowed
          //move_i32(x);
          assert_eq!(y.as_ref(), &42);
     }
     move_i32(x);
}

fn borrowing_vs_mutable_borrowing() {
     let x = Box::new(42);
     let mut y = Box::new(42);

     let _x_immutable = &x;
     let _y_immutable = &y;

     //Can't get &mut to immutable data
     //let x_mutable = &mut x;
     let y_mutable = &mut y;
     //Can't get two &mut to the same data
     //let y_mutable2 = &mut y;
     //Can't get immutable refs while a mutable ref exist
     // let y_immmutable = &y;

     assert_eq!(y_mutable.as_ref(), &42);

     let y_immutable = &y;

     //Can't get murable ref while immutable exist
     //let y_mutable_2 = &mut x;

     assert_eq!(y_immutable.as_ref(), &42);
}

fn ref_pattern() {
     let mut a = Box::new(32);
     //These two are the same
     let _a_ref_1 = &a;
     let ref _a_ref_2 = a;

     //These two are the same
     let _a_mutref_1 = &mut a;
     let ref mut _a_mutref_2 = a;

     //First, & and &mut on the left cat be used for destructuring
     // let &mut destructure = _a_mutref_2; // - this one will fail, can't move out

     //Second, ref and ref mut allow take fererence if only lvalue can be annotated
     let mut tuple = (34, 54);
     let (_, ref mut right) = tuple;
     *right = 4;
     assert_eq!(tuple.1, 4);
}

fn lifetimes() {
     //lifetime is determined by where variable is declared
     //scope os is determined by where variable is used

     //"<'a>": the lifetime of print_ref must not exceed the lifetime 'a
     //"x: &'a i32": the lifetime of 'a is constrained by &a
     //defaults to 'static if not constrained
     fn _print_ref<'a>(x: &'a i32) {
          println!("x is {}", x);
     }

     struct _Owner(i32);
     impl _Owner {
          fn _add_one<'a>(&'a mut self) {
               self.0 += 1
          }
     }

     struct _Borrowed<'a>(&'a i32);
     struct _NamedBorrowed<'a> {
          x: &'a i32,
          y: &'a i32,
     }
     enum _Either<'a> {
          Num(i32),
          Ref(&'a i32),
     }

     impl<'a> Default for _Borrowed<'a> {
          fn default() -> Self {
               Self(&10)
          }
     }

     //T: 'a  --  all references in T must outlive lifetime 'a
     //T: Trait + 'a  --  type T implements Trait and all references in T must outlive 'a
     fn _print_ref2<'a, T>(t: &'a T)
     where
          T: Debug + 'a,
     {
          println!("x is {:?}", t);
     }

     //A longer lifetime can be coerced into a shorter one
     //Here, two lifetimes are coerced into the shorter one
     fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
          first * second
     }
     //Here, longer lifetime is coerced into a shoerter one to return the value
     fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
          first
     }
     let first = 3; //longer lifetime
     {
          let second = 4; //shorter lifetime
          multiply(&first, &second);
          choose_first(&first, &second);
     }

     //'static
     //'static as a reference lifetime: data lives for the entire lifetime of the running program
     //1. make a constant with the static declaration
     static _NUM: i32 = 18;
     //2. make a string literal which has type [&'static str]
     let _static_string: &'static str = "read-only memory";
     //'static as a trait bound means the type does not contain any non-static references
}

pub fn main() {
     raii();
     ownership_and_moves();
     moving_vs_borrowing();
     borrowing_vs_mutable_borrowing();
     ref_pattern();
     lifetimes();
}
