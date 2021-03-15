fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    
    let factor = 7;
    
    rotate(&arr, factor);
}

fn rotate(array: &[i32], factor: i32) -> &[i32] {
    let append = if factor > 0 { false } else { true };
    let length: usize = array.len();
    // Remove sign
    let mut f: usize = if factor < 0 { !factor + 1 } else { factor } as usize;
    f = f % length;
    if length < 1 {
        return array;
    }
    let mut response: Vec<i32> = Vec::new();
    
    for i in 0..length {
        if append {
            if i < f {
                response.push(array[i + length - f]);
            } else {
                response.push(array[i - f]);
            }
        } else {
            if i < length - f {
                response.push(array[i + f]);
            } else {
                response.push(array[i + f - length]);
            }
        }
        println!("{}", response[i]);
    }
    
    return array;
}
