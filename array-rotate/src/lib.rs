pub fn rotate(array: &mut [i32], factor: i32) {
    let reverse = factor <= 0;
    let length: usize = array.len();
    if length < 1 {
        return;
    }

    // For negative numbers, use the unary NOT operator to flip all the bits.
    // e.g. (1001 -> 0110). In integers using two's compliment, flipping the
    // bits of a negative number gives the positive number - 1.
    // add 1 to the result of flipping all bits
    let mut f: usize = if factor < 0 { !factor + 1 } else { factor } as usize;

    // Convert the factor to a number in the range 0 - length.
    f %= length;

    let mut i = 0;
    // O(m*n) time complexity where m is the factor size normalized & n is the
    // size of the array
    // O(1) size complexity. in place rotation.
    // rotate the digits
    while i < f {
        // make space to move the number
        // let idx = if reverse { 0 } else { length - 1 };

        // Shift numbers
        // reverse:     Shift left
        // not reverse: Shift right
        let start = if reverse { 1 } else { length - 1 };
        let pred = |j| if reverse { j < length } else { j > 0 };
        let mut j = start;
        while pred(j) {
            let temp = std::mem::take(&mut array[j]);
            let idx_prev = j - 1;
            let b = std::mem::replace(&mut array[idx_prev], temp);

            array[j] = b;
            j = if reverse { j + 1 } else { j - 1 };
        }
        i += 1;
    }
}

#[cfg(test)]
mod test_rotate {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr = [];

        rotate(&mut arr, 2);

        let expected: &[i32] = &[];
        assert_eq!(expected, &arr);
    }

    #[test]
    fn test_single_element_array() {
        let mut arr = [1];

        rotate(&mut arr, 2);

        assert_eq!(&[1], &arr);
    }

    #[test]
    fn test_large_array() {
        let mut arr = [1, 2, 3, 4, 5, 6];

        rotate(&mut arr, -7);
        assert_eq!(&[2, 3, 4, 5, 6, 1], &arr);

        rotate(&mut arr, -3);
        assert_eq!(&[5, 6, 1, 2, 3, 4], &arr);

        let mut arr = [1, 2, 3, 4, 5, 6];

        rotate(&mut arr, 7);
        assert_eq!(&[6, 1, 2, 3, 4, 5], &arr);

        rotate(&mut arr, 3);
        assert_eq!(&[3, 4, 5, 6, 1, 2], &arr);
    }
}
