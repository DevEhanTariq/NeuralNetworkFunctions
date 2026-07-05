pub fn relu(m: Vec<f32>) -> Vec<f32> {
    let mut y: Vec<f32> = Vec::new();
    for x in m {
        if x <= 0.0 {
            y.push(0.0);
        } else {
            y.push(x);
        }
    }
    return y;
}

pub fn leaky_relu(m: Vec<f32>, a: f32) -> Vec<f32> {
    let mut y: Vec<f32> = Vec::new();
    for x in m {
        if x <= 0.0 {
            y.push(x*a);
        } else {
            y.push(x);
        }
    }
    return y;
}