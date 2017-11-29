fn main() {
    println!("Hello, world!");

    another_function();
    function_with_one_parameter(5);
    function_with_two_parameters(5, 6);
    an_expression();
    run_five();
    run_plus_one();
}

fn another_function() {
    println!("Another function.");
}

fn function_with_one_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_two_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn an_expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn run_five() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn run_plus_one() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
