// Remove Duplicates from Sorted Array
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/727/

// taken from the the solutions
fn remove_duplicates(nums: &mut [i32]) -> usize {
    // Done in place via 2 iterators
    if nums.is_empty() {
        return 0;
    }

    let mut k = 0;
    for i in 1..nums.len() {
        if nums[k] != nums[i] {
            k += 1;
            nums[k] = nums[i];
        }
    }
    k + 1
}

fn remove_duplicates_via_replace(nums: &mut Vec<i32>) -> usize {
    let mut v: Vec<i32> = Vec::with_capacity(nums.len());
    for num in nums.iter() {
        match v.last() {
            Some(&vlast) => {
                if vlast != *num {
                    v.push(*num);
                }
            }
            None => v.push(*num),
        }
    }
    if v.len() == nums.len() {
        return nums.len();
    }
    nums.clear();
    nums.append(&mut v);
    nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_validation(input: &mut Vec<i32>, expected: &Vec<i32>) {
        let k = remove_duplicates(input);
        assert_eq!(expected.len(), k);
        for i in 0..k {
            assert_eq!(expected[i], input[i]);
        }
    }

    #[test]
    fn empty_input() {
        let mut input = Vec::new();
        let expected = Vec::new();
        test_validation(&mut input, &expected);
    }

    #[test]
    fn single_input() {
        let mut input = vec![1];
        let expected = vec![1];
        test_validation(&mut input, &expected);
    }

    #[test]
    fn no_duplicates() {
        let mut input = vec![1, 2, 3, 4, 5, 6];
        let expected = vec![1, 2, 3, 4, 5, 6];
        test_validation(&mut input, &expected);
    }

    #[test]
    fn one_duplicate() {
        let mut input = vec![1, 1, 2, 3, 4, 5, 6];
        let expected = vec![1, 2, 3, 4, 5, 6];
        test_validation(&mut input, &expected);
    }

    #[test]
    fn multiple_duplicates() {
        let mut input = vec![1, 1, 2, 3, 4, 4, 4, 4, 5, 6];
        let expected = vec![1, 2, 3, 4, 5, 6];
        test_validation(&mut input, &expected);
    }
}
