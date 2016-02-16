/// mark a bomb square.
fn mark_bomb(row: usize, col: usize, board: & mut Vec<Vec<i8>>) -> ()
{
    // decrement bomb squares by 10, then increment all squares by 1 in 3x3 grid around bomb.
    if let Some(mut current_row) = board.get_mut(row){
        if let Some(mut cx) = current_row.get_mut(col) {
            *cx -= 10;
        }
    }

    for r in if row > 1 {row - 1} else {0} .. row + 2 {
        if let Some(mut update_row) = board.get_mut(r) {
            for c in if col > 1 {col - 1} else {0} .. col + 2 {
                if let Some(mut cx) = update_row.get_mut(c) {
                    *cx += 1;
                }
            }
        }
    }

}

/// Annotate a minesweeper board.
/// Assume a rectangular board, so all rows will have the same length.
/// "*" marks bombs, " " marks all other
pub fn annotate(bx: &Vec<&str>) -> Vec<String> {

    let max_row = bx.len();
    let max_col = bx[0].len();
    let mut board = vec![vec![0i8; max_col]; max_row];

    // mark all the bomb squares
    for (r, &row) in bx.iter().enumerate() {
        for (col, ch) in row.char_indices() {
            match ch {
                '*' => mark_bomb(r, col, & mut board),
                _ => (),
            }
        }
    }

    // convert arr to a Vec<String>
    board.iter().map(|r| r.iter()
                   .map(|&s| match s {
                       0 => " ".to_string(),
                       n if n > 0 => n.to_string(),
                       _ => "*".to_string(),
                   })
                   .collect::<Vec<String>>()
                   .join("")
                  ).collect::<Vec<String>>()
}

