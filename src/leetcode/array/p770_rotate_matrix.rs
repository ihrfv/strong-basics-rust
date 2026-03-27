struct Solution;

impl Solution {
    // Different order, more complicated solution
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::diagonal_mirror(matrix);
        Self::vertical_mirror(matrix);
    }

    fn rotate_optimal(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for i in 0..matrix.len() {
            for j in i + 1..matrix.len() {
                let tmp = matrix[j][i];
                matrix[j][i] = matrix[i][j];
                matrix[i][j] = tmp;
            }
        }
    }
    //
    fn diagonal_mirror(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let boundary = len - 1;

        for i in 0..len / 2 {
            for j in i..len - i - 1 {
                println!("i={i}, j={j}");
                println!("{:?}", matrix);
                // swap abowe diagonal
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[boundary - j][boundary - i];
                matrix[boundary - j][boundary - i] = tmp;
                println!("{:?}", matrix);
                // swap beyond diagonal
                if i != j {
                    let tmp = matrix[j][i];
                    matrix[j][i] = matrix[boundary - i][boundary - j];
                    matrix[boundary - i][boundary - j] = tmp;
                    println!("{:?}", matrix);
                }
            }
        }
    }

    fn vertical_mirror(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let boundary = len - 1;
        for j in 0..len {
            for i in 0..len / 2 {
                // swap
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[boundary - i][j];
                matrix[boundary - i][j] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn vertical_mirror_test() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]];

        Solution::vertical_mirror(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn diagonal_mirror_test() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![9, 6, 3], vec![8, 5, 2], vec![7, 4, 1]];

        Solution::diagonal_mirror(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn rotate_test() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        Solution::rotate(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn rotate_test_2() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];

        Solution::rotate(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn vertical_mirror_test_2() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected = vec![
            vec![15, 14, 12, 16],
            vec![13, 3, 6, 7],
            vec![2, 4, 8, 10],
            vec![5, 1, 9, 11],
        ];

        Solution::vertical_mirror(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn diagonal_mirror_test_2() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected = vec![
            vec![16, 7, 10, 11],
            vec![12, 6, 8, 9],
            vec![14, 3, 4, 1],
            vec![15, 13, 2, 5],
        ];

        Solution::diagonal_mirror(&mut input);

        assert_eq!(expected, input);
    }
    #[test]
    fn rotate_test_3() {
        let mut input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        Solution::rotate(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn vertical_mirror_test_3() {
        let mut input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let expected = vec![
            vec![13, 14, 15, 16],
            vec![9, 10, 11, 12],
            vec![5, 6, 7, 8],
            vec![1, 2, 3, 4],
        ];

        Solution::vertical_mirror(&mut input);

        assert_eq!(expected, input);
    }

    #[test]
    fn diagonal_mirror_test_3() {
        let mut input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let expected = vec![
            vec![16, 12, 8, 4],
            vec![15, 11, 7, 3],
            vec![14, 10, 6, 2],
            vec![13, 9, 5, 1],
        ];

        Solution::diagonal_mirror(&mut input);

        assert_eq!(expected, input);
    }
}
