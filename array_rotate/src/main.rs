// https://www.geeksforgeeks.org/array-rotation/
fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6];
    
    let factor = 7;
    
    left_rotate(&mut arr, factor);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn left_rotate(array: &mut [i32], factor: usize) {
    let length = array.len();
    if length < 1 {
        return;
    }

    let f = factor % length;
    let gcd = gcd(f, length);
    for i in 0..gcd {
        let tmp = array[i];
        let mut j = i;

        loop {
            let mut k = j + f;
            if k >= length {
                k -= length;
            }

            if k == i {
                break;
            }

            array[j] = array[k];
            j = k;
        }
        array[j] = tmp;
    }
}

#[cfg(test)]
mod tests {
    mod the_rotate_function {
        use crate::left_rotate;

        #[test]
        fn should_return_original_array_given_a_single_element_array() {
            let mut array = [2];

            left_rotate(&mut array, 2);

            assert_eq!(array[0], 2);
        }

        #[test]
        fn should_return_rotated_array() {
            let mut array = [1, 2, 3, 4, 5];

            left_rotate(&mut array, 2);

            assert_eq!(array[0], 3);
            assert_eq!(array[3], 1);
        }
    }
}