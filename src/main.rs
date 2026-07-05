mod math;

fn main() {
    println!("Hello, world!");
    let a = [1.0, 2.0, 3.0, 4.0, 5.0];
    let b = [1.0, 2.0, 3.0, 4.0, 5.0];
    let c = math::matrix_multiplication(&a, &b);
    println!("{:?}", c);
    let c = math::sum_list(c);
    println!("{}", c);
}
