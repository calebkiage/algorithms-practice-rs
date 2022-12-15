pub fn rotate(array: &[i32], factor: i32) -> Vec<i32> {
    let append = factor <= 0;
    let length: usize = array.len();
    if length < 1 {
        return Vec::with_capacity(0);
    }
    // Remove sign
    let mut f: usize = if factor < 0 { !factor + 1 } else { factor } as usize;
    f %= length;
    let mut response: Vec<i32> = Vec::with_capacity(length);

    for i in 0..length {
        if append {
            if i < f {
                response.push(array[i + length - f]);
            } else {
                response.push(array[i - f]);
            }
        } else if i < length - f {
            response.push(array[i + f]);
        } else {
            response.push(array[i + f - length]);
        }
    }

    response
}

#[cfg(test)]
mod test_rotate {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr = [];

        let result = rotate(&arr, 2);

        let expected: &[i32] = &[];
        assert_eq!(expected, result.as_slice());
    }

    #[test]
    fn test_single_element_array() {
        let arr = [1];

        let result = rotate(&arr, 2);

        assert_eq!(&[1], result.as_slice());
    }

    #[test]
    fn test_large_array() {
        let arr = [1, 2, 3, 4, 5, 6];

        let result = rotate(&arr, 7);

        assert_eq!(&[2, 3, 4, 5, 6, 1], result.as_slice());
    }
}
