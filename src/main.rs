enum CommonFunctions {
    Quadratic(f64),
    Cubic(f64),
    Exponential(f64),
    Sine(f64),
}

impl CommonFunctions {
    fn calculate(&self) -> f64 {
        match self {
            CommonFunctions::Quadratic(input) => input.powi(2),
            CommonFunctions::Cubic(input) => input.powi(3),
            CommonFunctions::Exponential(input) => input.exp2(),
            CommonFunctions::Sine(input) => input.sin(),
        }
    }
}

fn main() {
    let domain: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];

    for input in domain {
        println!("x : {input}");
        println!("x**2 = {}", CommonFunctions::Quadratic(input).calculate());
        println!("x**3 = {}", CommonFunctions::Cubic(input).calculate());
        println!("2**x = {}", CommonFunctions::Exponential(input).calculate());
        println!("sin(x) = {}", CommonFunctions::Sine(input).calculate());
    }
}
