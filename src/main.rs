use crate::binary_search::recursive_bin_search;
use crate::exponential_search::exponen_search;
use crate::linear_search::lin_search;

mod linear_search;
mod binary_search;
mod exponential_search;

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

mod questions {
    pub fn two_crystal_balls(){
        /*
        When given two crystal balls that will break if dropped from a
        high enough distance, determine the exact spot in which it will break
        in the most optimized way.
        */

        //false values upto a point true values follow
        //[false, false, false, false, false, true, true, true, ...]
        //move to the 1/2^N 
        let low = false;
        let high = true;




    }
}