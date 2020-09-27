
fn loop_loop() {
     //Infinite loop
     //break & continue with labels are available
     //might return a value with 'break 56'

     let mut x = 1;
     let mut count = 0;

     let p = 'bla: loop {
          count += 1;

          if count == 3 {
               x += 10;
               continue 'bla;
          }

          if count == 5 {
               break 'bla 42;
          }
     };

     assert_eq!(x, 11);
     assert_eq!(p, 42);
}

fn while_loop() {
     let mut k = 1;

     //continue/break with labels are available
     //() is always return value
     let ret = 'p: while k < 10 {
          k += 1;
          if k < 3 {
               continue 'p
          }
          break 'p;
     };

     assert_eq!(k, 3);
     assert_eq!(ret, ());
}

fn if_else() {
     let n = 4;

     //Else branch is required if expression needed (if w/o less evaluates to ())
     //All sub-blocks must be of the same type
     //might return a value

     let big_n = if n < 10 {
          n * 2
     } else if n < 20 {
          n * 3
     } else {
          n * 4
     };

     assert_eq!(big_n, 8);
}

fn for_range_loop() {
     //for range/iterator loop
     //continue/break with labels available
     //() is always return value

     //range with exclusive right bound
     let mut x = 0;
     for n in 1..10 {
          x += n;
     };
     assert_eq!(x, 45);

     //range with inclusive right bound
     let mut y = 0;
     for n in 1..=9 {
          y += n
     }
     assert_eq!(y, 45);

     //can use continue/break
     let mut z = 0;
     'l: for _ in 1..100 {
          z += 1;
          if z < 10 {
               continue 'l
          } else {
               break 'l
          }
     }
     assert_eq!(z, 10);

     //() is the return value
     let ret = for _ in 1..10 {
          4;
     };
     assert_eq!(ret, ());
}

fn for_iterator_loop() {
     {
          let names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names {
               names2.push(name)
          }
          //Can't do: values were moved
          // assert_eq!(names, names);
     }

     {
          let names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names.into_iter() {
               names2.push(name)
          }
          //Can't do: values were moved
          // assert_eq!(names, names);
     }
     
     {
          let names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names.iter() {
               names2.push(name)
          }
          //Can do: values were immutably borrowed
          assert_eq!(names, names);
          assert_eq!(names2, vec![&"Bob", &"Frank", &"Ferris"]);
     }

     {
          let mut names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names.iter_mut() {
               names2.push(name)
          }
          //Can do: values were mutably borrowed
          // assert_eq!(names, names);
          assert_eq!(names2, vec![&"Bob", &"Frank", &"Ferris"]);
     }
}


pub fn main() {
     if_else();
     loop_loop();
     while_loop();
     for_range_loop();
     for_iterator_loop();
}