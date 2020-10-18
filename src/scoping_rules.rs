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
     fn destroy(_: Box<i32>) {
     }
     
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

pub fn main() {
     raii();
     ownership_and_moves();
}