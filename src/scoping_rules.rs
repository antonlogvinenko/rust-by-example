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
     
}

pub fn main() {
     raii();
     
     ownership_and_moves();
     moving_vs_borrowing();
     borrowing_vs_mutable_borrowing();
     ref_pattern();
     
     lifetimes();
}
