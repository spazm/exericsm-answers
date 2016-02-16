fn bomb_at(row: usize, col: usize, max_row: usize, max_col: usize, arr: & mut Vec<Vec<i8>>) -> ()
{
    println!("row: {}, max_row: {}\ncol: {}, max_col: {}", row, max_row, col, max_col);

    // decrement bomb squares by 10, then increment all squares by 1 in 3x3 grid around bomb.
    if let Some(mut cur_row) = arr.get_mut(row){
        println!("cur_row: {:?}", &cur_row);
        {
        let mut a = cur_row.get_mut(col).unwrap();
        *a -= 10;
        }
        println!("cur_row: {:?}", &cur_row);
    }

    for r in if row > 1 { row - 1} else {0} .. row + 2 {
        if let Some(mut update_row) = arr.get_mut(r) {
            for c in if col > 1 { col - 1} else { 0} .. col + 2 {
                println!("update_row iter -> row:{}, col:{}", r, c);
                if let Some(mut cx) = update_row.get_mut(c) {
                    *cx += 1;
                }
            }
        }
    }
    println!("arr: {:?}", &arr);

}
/// Annotate a minesweeper board.
/// Assume a rectangular board, so all rows will have the same length.
pub fn annotate(board: &Vec<&str>) -> Vec<String> {
    let mut output = Vec::new();

    let max_row = board.len();
    let max_col = board[0].len();
    let mut arr = vec![vec![0i8; max_col]; max_row];
    println!("arr: {:?}", arr);
    println!("arr[0]: {:?}", arr[0]);

    //let v = t.and_then(|a| a + 2);
    //println!("t: {:?} , v: {:?}", t, v);
    //let width = board[0]
    //let grid = vec!(width, 0);
    for (r, &row) in board.iter().enumerate() {
        // each bomb can affect the count of 8 squares.  
        // increment the squares as we pass.
        for (col, ch) in row.char_indices() {
            match ch {
                '*' => bomb_at(r, col, max_row, max_col, & mut arr),
                _ => (),
            }
        }
        println!("row: {:?}", &row);
    }

    // convert arr to a Vec<String>
    for r in arr.iter() {
        output.push(
            r.iter().map(|&s| match s {
                0 => " ".to_string(),
                n if n < 0 => "*".to_string(),
                n => n.to_string()})
            .collect::<Vec<String>>().join(""));
    }
    output
}

