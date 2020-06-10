fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // nested
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if as an expression
    let cond = true;
    let _number = if cond { 5 } else { 6 };

    // endless loop - uncomment below
    // loop {
    //     println!("again!");
    // }

    // break expression to return and stop loop
    let mut counter = 0;

    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // while loops
    let mut number2 = 3;

    while number2 != 0 {
        println!("{}!", number2);

        number2 -= 1;
    }

    println!("LIFTOFF!!!");

    // iterate through a collection
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // reverse iterate
    for number3 in (1..4).rev() {
        println!("{}!", number3);
    }
    println!("LIFTOFF!!!");
}