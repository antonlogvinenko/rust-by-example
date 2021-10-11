//loop and if can return values
//can use * and & or ref and ref mut anywhere in matches
//can use @ to rename anywhere in matches

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
            continue 'p;
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
    }
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
            continue 'l;
        } else {
            break 'l;
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

fn match_control() {
    let number = 42;

    //pattern1 | ... | pattern if condition => ...
    //pattern is matched value, _, range, destructuring pattern
    //destructuring pattern = structure with: matched value | destrucrturing pattern
    //also, all matched constants can be named with "name @"
    let x = match number {
        1 => "one".to_owned(),
        2 | 3 | 4 | 5 => "two".to_owned(),
        6..=10 => ">= 4".to_owned(),
        n @ 11..=42 if n % 2 == 0 => format!("even {} >= 10", n),
        n @ 11..=42 if n % 2 == 1 => format!("odd {} >= 10", n),
        _ => "dunno".to_owned(),
        // p => "dunno".to_owned()
    };

    assert_eq!(x, "even 42 >= 10".to_owned());
}

fn match_and_destructuring() {
    //destructure tuples
    let pair = (2, false);
    let result = match pair {
        (2, true) => "a",
        (2, false) | (3, false) => "b",
        (_, true) => "c",
        (_d, g @ false) if g => "bla",
        _ => "oops",
    };
    assert_eq!(result, "b");

    //destructure structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    //can use field name to refer
    //can rename field name to refer
    //can destrcuture field
    //can ignore field(s) with ..
    let x = match foo {
        Foo { x: (1, b), y } => format!("{} {}", b, y).to_owned(),
        Foo { x: i, y: 2 } => format!("{:?}", i).to_owned(),
        Foo { x: (2, _), .. } => format!("ignored").to_owned(),
        _ => format!("anything else"),
    };
    assert_eq!(x, "2 3");

    //destructure enums
    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        RGB(u32, u32, u32),
    }
    let color = Color::RGB(1, 2, 3);
    let c = match color {
        Color::Red => "red".to_owned(),
        Color::Blue => "blue".to_owned(),
        Color::RGB(r, g, b) => format!("r{} g{} b{}", r, g, b),
    };

    assert_eq!(c, "r1 g2 b3");
}

fn if_let() {
    let x = Some(42);
    let y = !true;
    let result = if let Some(y) = x {
        format!("it's {}", y)
    } else if y {
        format!("can't happen")
    } else {
        format!("can't happen either")
    };
    assert_eq!(result, "it's 42");

    //Can be used when no PartialEq and can't compare
    #[derive(Debug)]
    enum Foo {
        Bar,
    }
    let bar = Foo::Bar;
    #[allow(irrefutable_let_patterns)]
    let result = if let f @ Foo::Bar = bar {
        format!("matched to {:?}", f)
    } else {
        format!("matched to smth else")
    };
    assert_eq!(result, "matched to Bar");
}

fn while_let() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i != 42 {
            optional = Some(i + 1);
        } else {
            optional = None
        }
    }
    assert_eq!(optional, None);
}

/**
 * Following * and &, mut and ref mut can be used anywhere in matching constructions listed in this file
 */
fn match_and_refrerences() {
    //destructure with * on value or with & on pattern
    {
        let reference = &4;
        let x = match reference {
            &val => format!("{}", val),
        };
        assert_eq!(x, "4");

        let x = match *reference {
            val => format!("{}", val),
        };
        assert_eq!(x, "4");
    }

    //creating immutable & mutable references
    {
        let value1 = 42;

        let c = match value1 {
            //note: * dereferencing is optional, present for demonstation purpose only
            ref x => format!("{}", *x),
        };
        assert_eq!(c, "42");

        let mut value2 = 42;

        let c = match value2 {
            ref mut x => {
                *x += 1;
                format!("{}", x)
            }
        };
        assert_eq!(c, "43");
    }
}

pub fn main() {
    if_else();
    loop_loop();
    while_loop();
    for_range_loop();
    for_iterator_loop();
    match_control();
    match_and_destructuring();
    if_let();
    while_let();
    match_and_refrerences();
}
