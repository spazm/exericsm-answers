pub fn square_of_sum(i:u64) -> u64 {
    let mut acum : u64 = 0;
    for k in (1 .. i + 1)
    {
        acum += k
    }
    acum * acum
}
pub fn sum_of_squares(i:u64) -> u64 {
    let mut acum : u64= 0;
    for k in (1 .. i + 1){
        acum += k*k
    }
    acum
}
pub fn difference(i:u64) -> u64 {
    square_of_sum(i) - sum_of_squares(i)
}
