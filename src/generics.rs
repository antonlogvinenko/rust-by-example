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

     fn red<T: Red>(_: &T) -> &'static str {
          "red"
     }
     fn blue<T: Blue>(_: &T) -> &'static str {
          "blue"
     }

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

/**
 * Where clause allows
 * 1. Writing generic types and bounds separately (might be clearer)
 * 2. Defining bounds for arbitrary types, not just type vairables
 */
fn where_clause() {
     //Defining generic types in where
     struct _YourType;
     trait TraitB {};
     trait TraitC {};
     trait TraitE {};
     trait TraitF {};
     trait MyTrait<X, Y> {};
     impl<A, D> MyTrait<A, D> for _YourType
     where
          A: TraitB + TraitC,
          D: TraitE + TraitF,
     {
     };

     //Defining bounds for Option<T> in where
     trait PrintInOption {
          fn print_in_option(self);
     }
     impl<T> PrintInOption for T
     where
          Option<T>: std::fmt::Debug,
     {
          fn print_in_option(self) {
               println!("{:?}", Some(self));
          }
     }
}

fn new_type_idiom() {
     struct Years(i32);
     struct Days(i32);

     fn old_enough(age: &Years) -> bool {
          age.0 >= 18
     }

     let _age_days = Days(4444);
     let age_years = Years(43);
     assert_eq!(true, old_enough(&age_years));
     //This won't work: types don't match
     // assert_eq!(true, old_enough(&_age_days));
}

fn associated_types() {
     struct Container(i32, i32);

     trait Contains {
          type A;
          type B;

          fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
          fn first(&self) -> i32;
          fn last(&self) -> i32;
     }

     impl Contains for Container {
          type A = i32;
          type B = i32;

          fn contains(&self, x: &i32, y: &i32) -> bool {
               (&self.0 == x) && (&self.1 == y)
          }

          fn first(&self) -> i32 {
               self.0
          }

          fn last(&self) -> i32 {
               self.1
          }
     }

     //Just any C that has trait Contains
     fn difference<C: Contains>(container: &C) -> i32 {
          container.last() - container.first()
     }

     let container = Container(1, 3);
     assert_eq!(2, difference(&container));
}

/*
 * Phantom type parameter is the one that doesn't affect runtime,
 * but is checked during compile time.
 * Used to enforce Units of measure, for instance.
 */
fn phantom_type_paramters() {
     use std::marker::PhantomData;

     struct PhantomTuple<A, B>(A, PhantomData<B>);
     struct PhantomStruct<A, B> {
          _first: A,
          phantom: PhantomData<B>,
     };

     let _t1: PhantomTuple<char, i32> = PhantomTuple('Q', PhantomData);
     let _t2: PhantomTuple<char, f64> = PhantomTuple('T', PhantomData);

     let _s1: PhantomStruct<char, i32> = PhantomStruct {
          _first: 'Q',
          phantom: PhantomData,
     };
     let _s2: PhantomStruct<char, f64> = PhantomStruct {
          _first: 'Q',
          phantom: PhantomData,
     };
}

pub fn main() {
     intro();
     functions();
     impls();
     traits();
     bounds();
     multiple_bounds();
     where_clause();
     new_type_idiom();
     associated_types();
     phantom_type_paramters();
}
