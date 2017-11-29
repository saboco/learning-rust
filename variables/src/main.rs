fn main() {
    println!("*********** Mutable variables ***********");
    mutable_variables();
    println!("*****************************************");
    println!("*************** Constants ***************");
    constants();
    println!("*****************************************");
    println!("*************** Shadowing ***************");
    shadowing();
    println!("*****************************************");
}

fn mutable_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn constants() {
    const MAX_POINTS: u32 = 100_000;

    println!("Max points is set to: {}", MAX_POINTS);
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
