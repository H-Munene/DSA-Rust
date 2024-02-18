
use crate::linear_search::lin_search;

mod linear_search;
mod binary_search;

fn main() {
    // let data = vec!['a', 'b', 'c'];
    // let target = 'c';
    // let answer = bin_search(&data, target).unwrap();

    let haystack = vec!["abs", "bcd", "dsa", "cde"];
    let needle = "dsa";

    let position = lin_search(&haystack, needle).unwrap();
    println!("Target {} found at index: {}",needle, position);

}
