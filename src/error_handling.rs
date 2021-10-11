fn panic() {
    //use env var RUST_BACKTRACE=0|1|full
    //panic!("bla");
}

fn option() {
    //Option: Some(T) | None
    let x: Option<i32> = Some(42);
    let y: Option<i32> = None;

    //unsafe: .unwrap => panic!
    // y.unwrap();
    assert_eq!(x.unwrap(), 42);

    //safe: pattern matching or ? operator
    fn increase1(x: Option<i32>) -> Option<i32> {
        let value = x?;
        Some(value + 1)
    }
    fn increase2(x: Option<i32>) -> Option<i32> {
        match x {
            Some(value) => Some(value + 1),
            None => None,
        }
    }
    assert_eq!(increase1(x), Some(43));
    assert_eq!(increase1(y), None);
    assert_eq!(increase2(x), Some(43));
    assert_eq!(increase2(y), None);

    //combining with map and map_then
    let x2 = x.map(|v| v + 1);
    let x3 = x.and_then(|v| Some(v));
    assert_eq!(x2, Some(43));
    assert_eq!(x3, Some(42));
}

fn result() {
    use std::error;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Copy)]
    struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for Error {};

    //Result: Ok(T) | Err(E)
    let x: Result<u32, Error> = Ok(42);
    let y: Result<u32, Error> = Err(Error);

    //unsafe: .unwrap => panic!
    assert_eq!(x, Ok(42));
    //assert_eq!(y.unwrap(), unimplemented!());

    //safe: pattern matching or ? operator
    fn increase1(x: Result<u32, Error>) -> Result<u32, Error> {
        match x {
            Ok(value) => Ok(value + 1),
            Err(err) => Err(err),
        }
    }
    fn increase2(x: Result<u32, Error>) -> Result<u32, Error> {
        let value = x?;
        Ok(value + 1)
    }
    assert_eq!(increase1(x), Ok(43));
    // assert_eq!(increase1(y).unwrap(), 42);
    assert_eq!(increase2(x), Ok(43));
    // assert_eq!(increase2(y), Ok(42));

    //combining with map and map_then
    let x2 = x.map(|v| v + 1);
    let y2 = y.map(|v| v + 1);
    assert_eq!(x2, Ok(43));
    assert_eq!(y2, Err(Error));
    let x3 = x.and_then(|v| Ok(v + 1));
    let y3 = y.and_then(|v| Ok(v + 1));
    assert_eq!(x3, Ok(43));
    assert_eq!(y3, Err(Error));

    //using Result in main
    // fn main() -> Result<(), Error>

    //type aliases
    // use std::io::Result;
    // type AliasedResult<T> = Result<T, Error>;

    //legacy: try? macro
    //try?(expr) == expr?
}

fn multiple_error_types() {
    use std::error;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Copy)]
    struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for Error {};

    //1. Converting Option to Result
    //option.ok_or
    let option1 = Some(32);
    let _result1 = option1.ok_or(Error);

    //2. Replacing different errors with one specific error
    //result.map_err
    let y = "cake".parse::<i32>().map_err(|_| Error);
    assert_eq!(y, Err(Error));

    //3.1. Boxing errors
    //error.into()
    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
    let y2: Result<i32> = "cake".parse::<i32>().map_err(|e| e.into());
    let y3: Result<i32> = Some(42).ok_or_else(|| Error.into());
    assert_eq!(y2.is_err(), true);
    assert_eq!(y3.is_ok(), true);

    //3.2. Boxing errors + ?
    fn auto_box() -> Result<i32> {
        let x = "cake".parse::<i32>()?;
        Ok(x + 1)
    }
    assert_eq!(auto_box().is_err(), true);

    //4. Wrapping errors + ?
    use std::num::ParseIntError;
    type WrappingResult<T> = std::result::Result<T, WrappingError>;
    #[derive(Debug)]
    enum WrappingError {
        Parse(ParseIntError),
    }
    impl From<ParseIntError> for WrappingError {
        fn from(err: ParseIntError) -> WrappingError {
            WrappingError::Parse(err)
        }
    }
    fn convert() -> WrappingResult<i32> {
        let parsed = "qweqew".parse::<i32>()?;
        Ok(2 * parsed)
    }
    let r: WrappingResult<i32> = convert();
    assert_eq!(r.is_err(), true);
}

fn iterating_over_results() {
    //Fail the entire operation
    let strings1 = vec!["tofu", "93", "18"];
    let _numbers1: Result<Vec<_>, _> = strings1.into_iter().map(|s| s.parse::<i32>()).collect();

    //Filter out failures
    let strings2 = vec!["tofu", "93", "18"];
    let _numbers2: Vec<_> = strings2
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    //Collect and group
    let strings = vec!["tofu", "93", "18"];
    let (_numbers, _errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
}

pub fn main() {
    panic();
    option();
    result();
    multiple_error_types();
    iterating_over_results();
}
