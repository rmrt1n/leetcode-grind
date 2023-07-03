use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for x in 0..9 {
        let mut h = vec![0; 9];
        let mut v = vec![0; 9];

        for y in 0..9 {
            if board[x][y] != '.' {
                h[board[x][y].to_digit(10).unwrap() as usize - 1] += 1;
            }

            if board[y][x] != '.' {
                v[board[y][x].to_digit(10).unwrap() as usize - 1] += 1;
            }
        }

        if h.into_iter().filter(|&i| i > 1).count() > 0 {
            return false;
        }

        if v.into_iter().filter(|&i| i > 1).count() > 0 {
            return false;
        }
    }

    for x in (0..9).step_by(3) {
        for y in (0..9).step_by(3) {
            let row1 = &board[x][y..(y + 3)];
            let row2 = &board[x + 1][y..(y + 3)];
            let row3 = &board[x + 2][y..(y + 3)];

            let mut count = vec![0; 9];
            let v = [row1, row2, row3]
                .concat()
                .into_iter()
                .filter(|&i| i != '.');

            v.for_each(|i| count[i.to_digit(10).unwrap() as usize - 1] += 1);

            if count.into_iter().filter(|&i| i > 1).count() > 0 {
                return false;
            }
        }
    }

    true
}

pub fn nc_is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut squares = HashSet::new();

    for x in 0..9 {
        for y in 0..9 {
            if board[x][y] == '.' {
                continue;
            }
            if !rows.insert((x, board[x][y])) {
                return false;
            }
            if !cols.insert((y, board[x][y])) {
                return false;
            }
            if !squares.insert(((x / 3, y / 3), board[x][y])) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(res, false);
    }

    #[test]
    fn test3() {
        let res = is_valid_sudoku(vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ]);
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test1() {
        let res = nc_is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test2() {
        let res = nc_is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test3() {
        let res = nc_is_valid_sudoku(vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ]);
        assert_eq!(res, false);
    }
}
