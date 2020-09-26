/**
 * Primitive types are converted to each other via casting.
 * Custom types (struct and enum) are converted using traits:
 * From/Into, and other specific ones.
 * 
 * TryFrom/TryInto are for failable conversions, hence Result return value
 * 
 * ToString: implement Display instead, ToString will use the same implementation
 * FromString: implemented for many times; arrange for type inference or specify type with turbofish syntax
 */

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

          fn try_from(value: i32) -> Result <Self, Self::Error> {
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
          radius: i32
     }

     impl fmt::Display for Circle {
          fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
               write!(f, "Circle of radius {}", self.radius)
          }
     }
     assert_eq!("Circle of radius 42", Circle {radius: 42}.to_string());

     let parsed: i32 = "5".parse().unwrap();
     let turbo_parsed = "10".parse::<i32>().unwrap();

     assert_eq!(parsed, 5);
     assert_eq!(turbo_parsed, 10);
}

pub fn main() {
     from_and_into();
     tryfrom_and_tryinto();
     to_and_from_string();
}
