DAY=$1
echo "use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!(\"../input/$DAY.txt\");
    // let input = "";

    // let mut grid = hashtag_grid(input);
    let parsed = input
        //.lines()
        //.chars()
        //.map(parse)
        //.map(|l| scanf!(l, \"\", ).unwrap())
        //.to_vec()
        //.sum::<isize>()
        ;
    
    //pv!(parsed);
    
}" > src/days/day_$DAY.rs

echo "#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_$DAY;
}

fn main() {
    days::day_$DAY::run();
}" > src/main.rs

touch src/input/$DAY.txt

if [ -d /mnt/c ]; then
    cmd.exe /c code src/days/day_$DAY.rs src/input/$DAY.txt
else
    code src/days/day_$DAY.rs src/input/$DAY.txt
fi
