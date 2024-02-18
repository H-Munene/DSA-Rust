use crate::binary_search::bin_search;

mod search::binary_search;

fn main() {
    let data = vec!['a', 'b', 'c'];
    let target = 'c';
    let answer = bin_search(&data, target).unwrap();

    println!("Found target : {} at index {} ", target, answer);
}
