// array input[] has the items to sort; array work[] is a work array
// https://en.wikipedia.org/wiki/Merge_sort#Bottom-up_implementation
fn bottom_up_merge_sort<'a>(input: &'a mut [i32], work: &'a mut [i32]) {
    let length = input.len();

    // Each 1-element run in input is already "sorted".
    // Make successively longer sorted runs of length 2, 4, 8, 16... until the whole array is sorted.
    let mut width = 1;
    while width < length {
        let mut i = 0;
        // Array input[] is full of runs of length width.
        let width_x2 = width << 1;
        while i < length {
            // Merge two runs: input[i:i+width-1] and input[i+width:i+2*width-1] to work[]
            // or copy input[i:length-1] to work[] ( if (i+width >= length) )
            // BottomUpMerge(A, i, min(i+width, n), min(i+2*width, n), B);
            merge(
                input,
                work,
                i,
                std::cmp::min(i + width, length),
                std::cmp::min(i + width_x2, length),
            );

            // Same as i + 2 * width
            i = i + width_x2;
        }

        // Now work array is full of runs of length 2*width.
        // Copy work array to array input for the next iteration.
        // A more efficient implementation would swap the roles of input and work.
        copy(input, work, length);
        // mem::swap doesn't work and swap_with_slice(&mut self, &mut [T]) is slow O(n) :-(
        // std::mem::swap(&mut input, &mut work);
        // Now array input is full of runs of length 2*width.
        // Same as 2 * width
        width = width_x2;
    }
}

fn merge(input: &[i32], output: &mut [i32], left_idx: usize, right_idx: usize, end_idx: usize) {
    let mut i = left_idx;
    let mut j = right_idx;

    for k in left_idx..end_idx {
        // If left run head exists and is <= existing right run head.
        if i < right_idx && (j >= end_idx || input[i] <= input[j]) {
            output[k] = input[i];
            i += 1;
        } else {
            output[k] = input[j];
            j += 1;
        }
    }
}

fn copy(dest: &mut [i32], src: &[i32], n: usize) {
    dest[..n].copy_from_slice(&src[..n])
}

#[cfg(test)]
mod test_merge_sort {
    #[cfg(test)]
    mod test_split_merge {
        use super::super::bottom_up_merge_sort;

        #[test]
        fn test_empty_array() {
            let mut src = [];
            let mut work = [];

            bottom_up_merge_sort(&mut src, &mut work);

            assert!(src.is_empty())
        }

        #[test]
        fn test_single_element_array() {
            let mut src = [1];
            let mut work = [0; 1];

            bottom_up_merge_sort(&mut src, &mut work);

            assert_eq!(src[0], 1);
        }

        #[test]
        fn test_small_arrays() {
            let mut input = [2, 1];
            let mut work = [0; 2];

            bottom_up_merge_sort(&mut input, &mut work);

            assert_eq!(&[1, 2], &input);

            let mut input = [4, 1, 2, 5];
            let mut work = [0; 4];

            bottom_up_merge_sort(&mut input, &mut work);

            assert_eq!(&[1, 2, 4, 5], &input);
        }

        #[test]
        fn test_unsorted_array() {
            let mut input = [2, 1, 10, 4, 4, 3, 7, 5];
            let mut work = [0; 8];

            bottom_up_merge_sort(&mut input, &mut work);

            assert_eq!(&[1, 2, 3, 4, 4, 5, 7, 10], &input);
        }

        #[test]
        fn test_reversed_array() {
            let mut input = [8, 7, 6, 5, 4, 3, 2, 1];
            let mut work = [0; 8];

            bottom_up_merge_sort(&mut input, &mut work);

            assert_eq!(&[1, 2, 3, 4, 5, 6, 7, 8], &input);
        }
    }

    mod test_merge {
        use super::super::merge;

        #[test]
        fn test_merge_both_empty() {
            let input = [];
            let mut output = [];

            merge(&input, &mut output, 0, 0, 0);

            assert_eq!(&[0; 0], &input);
        }

        #[test]
        fn test_merge_one_empty() {
            let input = [1];
            let mut output = [0; 1];

            merge(&input, &mut output, 0, 1, 1);

            assert_eq!(&[1], &output);
        }

        #[test]
        fn test_merge_one_element() {
            let input = [1, 2];
            let mut output = [0; 2];

            merge(&input, &mut output, 0, 1, 2);

            assert_eq!(&[1, 2], &output);
        }

        #[test]
        fn test_merge_two_elements() {
            let input = [1, 3, 2, 4];
            let mut output = [0; 4];

            merge(&input, &mut output, 0, 2, 4);

            assert_eq!(&[1, 2, 3, 4], &output);
        }
    }
}
