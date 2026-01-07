

fn my_sqrt(x: i32) -> i32 {
    let f = |y: f32| y * y - x as f32;
    let f_derivative = |y: f32| 2.0 * y as f32;

    let mut current_y = x as f32;

    for _ in 0..10 {
        current_y = current_y - f(current_y) / f_derivative(current_y);
    };

    current_y.floor() as i32
}

fn main() {
    let number = 64;
    let result = my_sqrt(number);
    println!("The integer square root of {} is {}", number, result); // Output should be 4
}