mod math;
mod activation_functions;

fn main() {
    println!("Hello, world!");

    let a: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let b: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    let c = math::matrix_multiplication(a, b);
    println!("{:?}", c);

    let c = math::sum_list(c);
    println!("{}", c);

    let c = vec![1.0, 2.0, -3.0, 4.0, -5.0];
    let c = activation_functions::leaky_relu(c, 0.01);
    println!("{:?}", c);
}
