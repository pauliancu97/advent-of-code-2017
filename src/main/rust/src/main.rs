mod utils;
mod day_ten;
mod day_eleven;
mod day_twelve;
mod day_thirteen;
mod day_fourteen;
mod day_sixteen;
mod day_seventeen;
mod day_nineteen;
mod day_twentyone;
mod day_twelve_2016;
mod day_thirteen_2016;
mod day_fifteen_2016;
mod day_sixteen_2016;
mod matrix;

fn main() {
    let answer = day_sixteen_2016::solve_part_one("01111001100111011", 35651584);
    println!("{}", answer);
}
