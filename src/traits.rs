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
    assert_eq!(dolly.is_naked(), false);
    assert_eq!(dolly.name, "Dolly");

    dolly.noise();
    dolly.talk();
}

fn derive() {
    #[derive(PartialEq, PartialOrd)]
    struct Centimiters(f64);

    let x1 = Centimiters(1.0);
    let x2 = Centimiters(2.0);
    assert_eq!(x1 < x2, true);
}

fn operator_overloading() {
    use std::ops;

    struct Foo;
    struct Bar;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    #[derive(PartialEq, Debug)]
    struct FooBar;

    let foo = Foo;
    let foobar = foo + Bar;

    assert_eq!(foobar, FooBar);
}

fn drop() {
    struct Droppable {
        name: &'static str,
    };

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    let _d = Droppable { name: "asd" };
}

fn iterators() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    //using interface directly
    let mut sequence = 0..3;
    assert_eq!(sequence.next().unwrap(), 0);
    assert_eq!(sequence.next().unwrap(), 1);

    //using interface indirectly
    let mut x = 0;
    for _ in 0..3 {
        x += 1;
    }
    assert_eq!(x, 3);

    //using manually defined interface directly
    let fib = Fibonacci { curr: 0, next: 1 };
    let p = fib.skip(3).take(1).next().unwrap();
    assert_eq!(p, 3);

    //using manually defined interface indirectly
    for x in (Fibonacci { curr: 0, next: 1 }).take(3) {
        println!("{}", x);
    }

    let array = [1u32, 3, 3, 7];
    let mut y = 0;
    for i in array.iter().skip(1).take(2) {
        y += i;
    }
    assert_eq!(y, 6);
}

fn impl_trait() {
    use std::iter;
    use std::vec::IntoIter;

    //1. shorter types
    fn _combine_vecs_explicit(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    fn combine_vecs_implicit(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    //2.1 types that can't be written: closures
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];

    let mut v3 = combine_vecs_implicit(v1, v2);
    assert_eq!(v3.next().unwrap(), 1);

    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| x + y;
        closure
    }
    assert_eq!(make_adder_function(5)(4), 9);

    //2.2 types that can't be written: intermediate iterators
    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
    }
    assert_eq!(double_positives(&vec![1, 2, 3]).nth(1).unwrap(), 4);
}

fn clone() {
    #[derive(Debug, PartialEq)]
    struct Unit1;
    let unit11 = Unit1;
    let _unit12 = unit11;
    //Can't use unit11: it was moved
    // assert_eq!(unit11, Unit1);

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Unit2;
    let unit21 = Unit2;
    let _unit22 = unit21;
    //Can use unit21: it was copied, not moved
    assert_eq!(unit21, Unit2);

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Unit3;
    let unit31 = Unit3;
    let _unit32 = unit31.clone();
    //Can use unit31: it was cloned, not moved
    assert_eq!(unit31, Unit3);
}

fn supertraits() {
    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn _cake(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
}

fn overlapping_traits() {
    struct Form {
        username: String,
        age: u8,
    }

    trait UsernameWidget {
        fn get(&self) -> String;
    }

    trait AgeWidget {
        fn get(&self) -> u8;
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    assert_eq!(<Form as UsernameWidget>::get(&form), "rustacean");
    assert_eq!(<Form as AgeWidget>::get(&form), 28);

    assert_eq!(UsernameWidget::get(&form), "rustacean");
    assert_eq!(AgeWidget::get(&form), 28);
}

pub fn main() {
    idea();
    derive();
    operator_overloading();
    drop();
    iterators();
    impl_trait();
    clone();
    supertraits();
    overlapping_traits();
}
