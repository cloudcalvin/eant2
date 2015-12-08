pub fn reverse<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    if vec.len() == 0 {
        panic!("length was 0");
    }
    let mut new = Vec::new();
    let mut index = vec.len() - 1usize;
    loop {
        new.push(vec[index].clone());
        if index == 0usize {
            break;
        }
        index -= 1usize;
    }
    new
}

pub fn sum_vec(vec: &Vec<f64>) -> f64 {
    let mut total = 0.0;
    for i in 0..vec.len() {
        total += vec[i];
    }
    total
}
