pub fn matrix_multiplication(a: Vec<f32>, b: Vec<f32>) ->  Vec<f32> {
    let mut z: Vec<f32> = Vec::new();
    for (x, y) in a.iter().zip(b.iter()) {
        z.push(x * y);
    }
    return z;
}

pub fn sum_list(a: Vec<f32>) -> f32 {
    let mut z: f32 = 0.0;
    for x in a {
        z += x;
    }
    return z;
}