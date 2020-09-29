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

          fn destroy(self) {

          }
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

}

pub fn main() {
     methods();
     closures();
}