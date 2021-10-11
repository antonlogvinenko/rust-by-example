/**
 * Structures: no args, unnamed args, named args
 * Constructing all three
 * Descrutcuring all three
 * Accessing all three
 *
 * Enums, +structures
 *
 * Custom types (struct and enum) are converted using traits:
 * From/Into, and other specific ones.
 * TryFrom/TryInto are for failable conversions, hence Result return value
 * ToString: implement Display instead, ToString will use the same implementation
 * FromString: implemented for many times; arrange for type inference or specify type with turbofish syntax
 *
 * Pattern matching
 * Aliases
 *
 * C-like structures
 */

fn structures() {
    #[derive(Debug, PartialEq)]
    struct Unit;

    #[derive(Debug)]
    struct Pair(i32, f32);

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    //Trivial structure
    assert_eq!(Unit, Unit);

    let pair = Pair(1, 2.0);
    assert_eq!(pair.1, 2.0);
    //Destructuring the structure
    let Pair(car, cdr) = pair;
    assert_eq!(car, 1);
    assert_eq!(cdr, 2.0);

    let point = Point { x: 1.0, y: 2f32 };
    //Struct update syntax
    let point2 = Point { x: 1.0, ..point };
    //Accessing the structure
    assert_eq!(point2.x, 1.0);
    assert_eq!(point2.y, 2f32);
    //Destructuring the structure
    let Point { x: left, y: right } = point;
    assert_eq!(left, 1.0);
    assert_eq!(right, 2f32);
}

fn enums() {
    #[derive(Debug)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) -> String {
        match event {
            WebEvent::PageLoad => "page loaded".to_owned(),
            WebEvent::PageUnload => "page unloaded".to_owned(),
            WebEvent::KeyPress(c) => format!("key {} pressed", c),
            WebEvent::Paste(paste) => format!("pasted string {}", paste),
            WebEvent::Click { x, y } => {
                format!("Clicked at ({}, {})", x, y)
            }
        }
    }

    assert_eq!(inspect(WebEvent::PageLoad), "page loaded");
    assert_eq!(inspect(WebEvent::PageUnload), "page unloaded");
    assert_eq!(inspect(WebEvent::KeyPress('k')), "key k pressed");
    assert_eq!(
        inspect(WebEvent::Paste("text".to_owned())),
        "pasted string text"
    );
    assert_eq!(
        inspect(WebEvent::Click { x: 32, y: 4 }),
        "Clicked at (32, 4)"
    );
}

fn from_and_into() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    assert_eq!(Number { value: 42 }.value, Number::from(42).value);

    //The Into trait is the reciorical of the Form trait
    let to: Number = 42.into();
    assert_eq!(42, to.value);
}

fn tryfrom_and_tryinto() {
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(42), Ok(EvenNumber(42)));
    assert_eq!(EvenNumber::try_from(43), Err(()));
}

fn to_and_from_string() {
    use std::fmt;

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    assert_eq!("Circle of radius 42", Circle { radius: 42 }.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    assert_eq!(parsed, 5);
    assert_eq!(turbo_parsed, 10);
}

fn enum_aliases() {
    #[derive(Debug, PartialEq)]
    enum AdfsdfsdfLsdfdsffgsdfsfasafJfsdfsdfFactoryBuilder {
        Add,
    }
    type FactoryBuilder = AdfsdfsdfLsdfdsffgsdfsfasafJfsdfsdfFactoryBuilder;
    let x = FactoryBuilder::Add;
    assert_eq!(x, AdfsdfsdfLsdfdsffgsdfsfasafJfsdfsdfFactoryBuilder::Add);
}

fn c_like_enums() {
    #[derive(Debug, PartialEq)]
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    assert_eq!(Color::Red as i32, 0xff0000);
    assert_eq!(Color::Blue as i32, 0x0000ff);
    assert_eq!(Color::Green as i32, 0x00ff00);
}

fn constants() {
    const THRESHOLD: i32 = 42;
    assert_eq!(THRESHOLD, 42);

    static LANGUAGE: &str = "Rust";
    assert_eq!(LANGUAGE, "Rust");
}

pub fn main() {
    structures();
    enums();
    from_and_into();
    tryfrom_and_tryinto();
    to_and_from_string();
    enum_aliases();
    c_like_enums();
    constants();
}
