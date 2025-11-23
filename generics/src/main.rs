use num_traits::ToPrimitive;

/// - `T: ToPrimitive, U: ToPrimitive` - accepts any types that can convert to primitive numbers
/// - Works with i32, f64, u16, etc. - any type implementing ToPrimitive trait
/// - Converts both inputs to f64 for consistent floating-point math
fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: i32 = 3;
    let b: f64 = 4.0;

    // Generics can be explicit like this - specify types at call site
    println!("{}", solve::<i32, f64>(a, b)); // Explicit: <i32, f64>

    // or implicit - Rust infers types from arguments
    println!("{}", solve(a, b)); // Implicit: compiler figures out <i32, f64>
}
