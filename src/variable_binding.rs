/**
 * mut means mutable
 * _x: suppress 'unused variable' warning
 * might be declared and initialized later
 * variables are shadowed in scope
 */

fn intro() {
    //With type annotation
    let x = 4;
    assert_eq!(x, 4);

    //Without type annotation
    let y: i64 = 5;
    assert_eq!(y, 5);

    //Prefix suppresses "unused variable" warning message
    let _a = 4;
}

fn mutability() {
    let _x = 1;
    //would not compile with an error: _x is immutable
    //_x += 1;

    let mut y = 1;
    y += 1;
    assert_eq!(2, y);
}

/*
 * Variable bindings have a scope, and are constrained to live in a block.
 * A block is a collection of statements enclosed by braces {}
 */
fn scope_and_shadowing() {
    let x = 1;
    {
        let x = 2;
        let _y = 3;
        assert_eq!(2, x);
    }
    //would not compile with an error: _y is out of scope
    // assert_eq!(3, _y);
    assert_eq!(1, x);
}

fn declare_first() {
    let x;

    {
        x = 1;
    }

    assert_eq!(1, x);
}

fn freezing() {
    let mut x = 1;
    {
        let x = 3;
        //would not compile with an error
        // x += 3;
        assert_eq!(3, x);
    }
    x += 7;
    assert_eq!(8, x);
}

pub fn main() {
    intro();
    mutability();
    scope_and_shadowing();
    declare_first();
    freezing();
}
