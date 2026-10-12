pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut hash = vec![0; 9];
            for j in 0..9 {
                let field = board[i][j];
                if field != '.' {
                    if hash[field.to_digit(10).unwrap() as usize - 1] == 1 {
                        return false;
                    } else {
                        hash[field.to_digit(10).unwrap() as usize - 1] += 1;
                    }
                }
            }
        }

        for j in 0..9 {
            let mut hash = vec![0; 9];
            for i in 0..9 {
                let field = board[i][j];
                if field != '.' {
                    if hash[field.to_digit(10).unwrap() as usize - 1] == 1 {
                        return false;
                    } else {
                        hash[field.to_digit(10).unwrap() as usize - 1] += 1;
                    }
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut hash = vec![0; 9];
                for k in 0..3 {
                    for l in 0..3 {
                        let field = board[i*3 + k][j*3 + l];
                        if field != '.' {
                            if hash[field.to_digit(10).unwrap() as usize - 1] == 1 {
                                return false;
                            } else {
                                hash[field.to_digit(10).unwrap() as usize - 1] += 1;
                            }
                        }                        
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn board(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn valid_board() {
        let input = board(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(Solution::is_valid_sudoku(input));
    }

    #[test]
    fn duplicate_in_row() {
        let input = board(&[
            "53..7...5",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!Solution::is_valid_sudoku(input));
    }

    #[test]
    fn duplicate_in_column() {
        let input = board(&[
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "5...8..79",
        ]);
        assert!(!Solution::is_valid_sudoku(input));
    }

    #[test]
    fn duplicate_in_sub_box() {
        let input = board(&[
            "83..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(!Solution::is_valid_sudoku(input));
    }

    #[test]
    fn empty_board_is_valid() {
        let input = board(&[
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ]);
        assert!(Solution::is_valid_sudoku(input));
    }
}
