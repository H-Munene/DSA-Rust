use crate::binary_search::recursive_bin_search;
use crate::exponential_search::exponen_search;
use crate::linear_search::lin_search;

mod linear_search;
mod binary_search;
mod exponential_search;
mod tests;
mod questions;

fn main() {
    // let data = vec!['a', 'b', 'c'];
    // let target = 'c';
    // let answer = bin_search(&data, target).unwrap();

    let haystack = vec!["abs", "bcd", "cde", "dsa"];
    let needle = "dsa";
    let pos2 = recursive_bin_search(&haystack, needle, 0, haystack.len()-1).unwrap();
    let po3  = exponen_search(&haystack, needle).unwrap();

    let position = lin_search(&haystack, needle).unwrap();
    println!("Target {} found at index: {}",needle, position);
    println!("Target {} found at index: {}",needle, pos2);
    println!("Target {} found at index: {}",needle, po3);

}
