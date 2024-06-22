const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn basic_variables() {
    println!("Basic variables...");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    println!("Constants...");
    println!("Constant number is {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    println!("Shadowing...");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn numbers() {
    println!("Numbers...");
    let a = 0;
    let b: i8 = 1;
    let c: i16 = 2;
    let d: i32 = 3;
    let e: i64 = 4;
    let f: i128 = 5;

    let bb: u8 = 1;
    let cc: u16 = 2;
    let dd: u32 = 3;
    let ee: u64 = 4;
    let ff: u128 = 5;

    let fa = 1.0;
    let fb: f32 = 3.0;

    println!("All numbers: {a} {b} {c} {d} {e} {f} {bb} {cc} {dd} {ee} {ff} {fa} {fb}");
}

fn math_operations() {
    println!("Math operations...");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 3;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("Math operations: {sum} {difference} {product} {quotient} {truncated} {remainder}");
}

fn booleans() {
    println!("Booleans...");
    let t = true;
    let f: bool = false;

    println!("Booleans: {t} {f}");
}

fn characters() {
    println!("Characters...");
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("Characters: {c} {z} {heart_eyed_cat}");
}

fn tuples() {
    println!("Tuples...");
    // A tuple can contain different data types
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values are {x} {y} {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("Accessing individual items in tuple {five_hundred} {six_point_four} {one}");
}

fn arrays() {
    println!("Arrays...");
    // An array must contain the same data type
    let a = [1, 2, 3, 4, 5];
    let [b, c, d, e, f] = a;
    println!("{b} {c} {d} {e} {f}");

    // Declare repeated values
    let multiple_three = [3; 5];
    for item in multiple_three {
        print!("{item} ");
    }
    println!("");

    // Accessing an array item
    let first = a[0];
    println!("First element {first}");
}

fn statement(value: u32, unit_label: char) {
    println!("Statement...");
    // A statement is anything that doesn't return a value (like this function doesn't return any value).
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    println!("Expression...");
    // An expression is anything that returns a value (e.g., let x = 2 + 1, let y = add(1, 2))
    // A new scope block ({}) is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn return_fn() -> i32 {
    println!("Function return...");
    // A short hand of return 5; (by not putting a semicolon, it is "return")
    5
}

fn if_else() {
    println!("If else...");
    let number = 3;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let() {
    println!("If in let...");
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn using_loop() {
    println!("Using loop...");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // "break" will return the result in loop
            break counter * 2;
        }
    };

    println!("The result is {result}")
}

fn labeling_loop() {
    println!("Labeling loop...");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // This will break inner loop
                break;
            }
            if count == 2 {
                // This will break counting_up loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn using_while() {
    println!("Using while...");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn using_for() {
    println!("Using for...");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn reverse_list() {
    println!("Reverse list...");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    basic_variables();
    constants();
    shadowing();
    numbers();
    math_operations();
    booleans();
    characters();
    tuples();
    arrays();
    statement(5, 'h');
    expression();
    println!("The value of fn is {}", return_fn());
    if_else();
    if_in_let();
    using_loop();
    labeling_loop();
    using_while();
    using_for();
    reverse_list();
}
