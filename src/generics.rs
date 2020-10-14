fn intro() {
     struct A;
     struct SingletonGen<T>(T);

     let _a = SingletonGen(42);
     let _b = SingletonGen("asd");
     let _c = SingletonGen(A);
}

fn functions() {
     struct A;
     struct SingletonGen<T>(T);

     fn gen_specialized_i32(_t: SingletonGen<i32>) {}

     fn gen_specialized_a(_t: SingletonGen<A>) {}

     fn gen<T>(_t: SingletonGen<T>) {}

     let _a = SingletonGen(42);
     gen_specialized_i32(_a);

     let _b = SingletonGen(A);
     gen_specialized_a(_b);

     let _c = SingletonGen(42);
     let _d = SingletonGen(A);
     gen(_c);
     gen(_d);
}

fn impls() {
     struct GenVal<T>(T);
     impl<T> GenVal<T> {
          fn value(&self) -> &T {
               &self.0
          }
     }

     let v = GenVal(32);
     assert_eq!(v.value(), &32);
}

fn traits() {
     struct Empty;
     struct Null;

     trait DoubleDrop<T> {
          fn double_drop(self, _: T);
     }

     impl<T, U> DoubleDrop<T> for U {
          fn double_drop(self, _: T) {}
     }

     let empty = Empty;
     let null = Null;

     empty.double_drop(null);
}

/**
 * Bounds allow to:
 * 1. Restrict generic type to types that satisfy bounds
 * 2. Access methods of traits specified in bounds
 * 3. Marker traits
 */
fn bounds() {
     //Syntax
     use core::fmt::Display;
     #[allow(dead_code)]
     fn printer<T: Display>(t: T) {
          println!("{}", t);
     }

     //Application 1: restrict to types that satisfy bounds
     struct _S<T: Display>(T);
     //Not allowed!
     // let s = _S(vec![1]);

     //Applciation 2: access methods of traits specified in bounds
     use std::fmt::Debug;
     trait HasArea {
          fn area(&self) -> f64;
     }
     impl HasArea for Rectangle {
          fn area(&self) -> f64 {
               self.length * self.height
          }
     }

     #[derive(Debug)]
     struct Rectangle {
          length: f64,
          height: f64,
     }
     #[allow(dead_code)]
     struct Triangle {
          length: f64,
          height: f64,
     }

     fn pring_debug<T: Debug>(t: &T) {
          println!("{:?}", t);
     }

     fn area<T: HasArea>(t: &T) -> f64 {
          t.area()
     }

     let rectangle = Rectangle {
          length: 2.3,
          height: 4.5,
     };
     let _triangle = Triangle {
          length: 2.2,
          height: 4.4,
     };

     area(&rectangle);
     pring_debug(&rectangle);

     // area(&_triangle);
     // pring_debug(&_triangle);

     //Application 3
     trait Red {}
     trait Blue {}

     struct Cardinal;
     struct BlueJay;
     struct Turkey;

     impl Red for Cardinal {}
     impl Blue for BlueJay {}

     fn red<T: Red>(_: &T) -> &'static str { "red" }
     fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

     let cardinal = Cardinal;
     let blue_jay = BlueJay;
     let _turkey = Turkey;

     red(&cardinal);
     blue(&blue_jay);
     //Not allowed
     //red(&_turkey);
     //Not allowed
     //blue(&blue_jay);
}

fn multiple_bounds() {
     use std::fmt::{Debug, Display};

     fn compare_prints<T: Debug + Display, U: Debug + Display>(t: &T, u: &U) {
          println!("{} {:?}", t, t);
          println!("{} {:?}", u, u);
     }

     let string = "cake";
     let int = 42;
     compare_prints(&string, &int);
}

pub fn main() {
     intro();
     functions();
     impls();
     traits();
     bounds();
     multiple_bounds();
}
