fn normalize_factor(factor: i32, maximum: usize) -> usize {
    // For negative numbers, use the unary NOT operator to flip all the bits.
    // e.g. (1001 -> 0110). In integers using two's compliment, flipping the
    // bits of a negative number gives the positive number - 1.
    // add 1 to the result of flipping all bits
    let f: usize = if factor < 0 { !factor + 1 } else { factor } as usize;

    // Convert the factor to a number in the range 0 - length.
    f % maximum
}

pub fn rotate(array: &mut [i32], factor: i32) {
    let reverse = factor <= 0;
    let length: usize = array.len();
    if length < 1 {
        return;
    }

    let f = normalize_factor(factor, length);

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

pub fn rotate_efficient(array: &mut [i32], factor: i32) {
    let reverse = factor <= 0;
    let length: usize = array.len();
    if length < 1 {
        return;
    }

    let f = normalize_factor(factor, length);
    let f = if reverse { f } else { length - f };

    let reverse = |arr: &mut [i32], start: usize, end: usize| {
        let mut start = start;
        let mut end = end;
        while start < end {
            let temp = arr[start];
            arr[start] = arr[end];
            arr[end] = temp;

            start += 1;
            end -= 1;
        }
    };

    reverse(array, 0, f - 1);
    reverse(array, f, length - 1);
    reverse(array, 0, length - 1);
}

#[cfg(test)]
mod test_rotate {
    use super::*;

    macro_rules! test_rotate_fn {
        ($expected:expr=> $fn:ident($array:expr, $factor:literal)) => {
            $fn(&mut $array, $factor);
            assert_eq!($expected, &$array);
        };
        ($expected:expr, $fn:ident, $array:expr, $factor:literal) => {
            $fn(&mut $array, $factor);
            assert_eq!($expected, &$array);
        };
    }

    #[test]
    fn test_empty_array() {
        let expected: &[i32] = &[];
        test_rotate_fn!(expected => rotate([], 2));
        test_rotate_fn!(expected, rotate_efficient, [], 2);
    }

    #[test]
    fn test_single_element_array() {
        test_rotate_fn!(&[1], rotate, [1], 2);
        test_rotate_fn!(&[1], rotate_efficient, [1], 2);
    }

    #[test]
    fn test_large_array() {
        let arr = [1, 2, 3, 4, 5, 6];

        {
            let mut arr = arr;
            test_rotate_fn!(&[2, 3, 4, 5, 6, 1], rotate, arr, -7);
            test_rotate_fn!(&[5, 6, 1, 2, 3, 4], rotate, arr, -3);
        }
        {
            let mut arr = arr;
            test_rotate_fn!(&[6, 1, 2, 3, 4, 5], rotate, arr, 7);
            test_rotate_fn!(&[3, 4, 5, 6, 1, 2], rotate, arr, 3);
        }

        {
            let mut arr = arr;
            test_rotate_fn!(&[2, 3, 4, 5, 6, 1], rotate_efficient, arr, -7);
            test_rotate_fn!(&[5, 6, 1, 2, 3, 4], rotate_efficient, arr, -3);
        }
        {
            let mut arr = arr;
            test_rotate_fn!(&[6, 1, 2, 3, 4, 5], rotate_efficient, arr, 7);
            test_rotate_fn!(&[3, 4, 5, 6, 1, 2], rotate_efficient, arr, 3);
        }
    }
}
