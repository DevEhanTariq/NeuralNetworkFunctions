fn matrix_multiplication(a: &[f32], b: &[f32]) ->  Vec<f32> {
    let mut z: Vec<f32> = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        z.push(x * y);
    }
    return z;
}

fn sum_list(a: Vec<f32>) -> f32 {
    let mut z: f32 = 0.0;
    for x in a {
        z += x;
    }
    return z;
}

fn main() {
    println!("Hello, world!");
    let a = [1.0, 2.0, 3.0, 4.0, 5.0];
    let b = [1.0, 2.0, 3.0, 4.0, 5.0];
    let c = matrix_multiplication(&a, &b);
    println!("{:?}", c);
    let c = sum_list(c);
    println!("{}", c);
}
