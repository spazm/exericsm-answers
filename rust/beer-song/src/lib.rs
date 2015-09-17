/// Sing verses of 99 bottles of beer song from `start` to `end`

pub fn sing(start: u32, end: u32) -> String {
    assert!(start > end);
    let mut response = Vec::with_capacity((start - end + 1) as usize);
    for n in (end .. start + 1).rev(){
        response.push(verse(n));
    }
    response.connect("\n")
}

/// One verse of the 99 bottles of beer song
pub fn verse(n: u32) -> String {
    match n {
        n if n > 2 => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {m} bottles of beer on the wall.\n",n=n, m=n-1),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => unreachable!()
    }
}



/*
3 bottles of beer on the wall, 3 bottles of beer.
Take one down and pass it around, 2 bottles of beer on the wall.

2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.

1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.

No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
*/
