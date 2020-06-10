fn main () {
    // celsius to fahrenheit
    let celsius = 15.0;

    let fahrenheit = convert_to_fahr(celsius);

    println!("Fahrenheit is: {}", fahrenheit);

    // fahrenheit to celsius
    let fahr = 78.3;

    let cel = convert_to_celsius(fahr);

    println!("Celsius is: {}", cel);

    // fibonacci
    let n = 11;

    let fibonacci_sum = generate_fibonacci(n);

    println!("Fibonacci sum is: {}", fibonacci_sum)
}

// Conversion functions

fn convert_to_celsius ( x: f32 ) -> f32 {
    return ( x - 32.0 ) * (0.5556);
}

fn convert_to_fahr ( x: f32 ) -> f32 {
    return ( x * 1.8 ) + 32.0;
}

// Fibonacci

fn generate_fibonacci ( n: i32 ) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("Zero is not a correct argument");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    return sum;
}