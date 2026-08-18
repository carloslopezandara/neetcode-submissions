impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![false;9];9];
        let mut cols = vec![vec![false;9];9];
        let mut boxes = vec![vec![false;9];9];
        let mut res = true;
        for row in 0..9 {
            for col in 0..9 {
                let value = board[row][col];

                if value == '.' {
                    continue;
                }

                let num = (value as u8 - b'1') as usize;

                let board_index = (row / 3) * 3 + (col /3);

                if rows[row][num] || cols[col][num] || boxes[board_index][num] {
                    return false;
                }

                rows[row][num] = true;
                cols[col][num] = true;
                boxes[board_index][num] = true;
            }
        }
        res
    }
}
