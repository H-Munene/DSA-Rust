/*
    binary search function implementing trait bounds and generic types
 */

//iterative implementation
pub fn bin_search<T> (data : &[T], target : T) -> Option<usize>
    where T : Copy + PartialOrd
{
    //initiate pointers
    let mut low  = 0;
    let mut high  = data.len() - 1;

    while low <= high {
        let mid = low + (high -low) /2;

        if target == data[mid] {
            return Some(mid);
        }else if target > data[mid] {
            //right of mid
            low = mid + 1;

        }else {
            //left of mid
            high = mid - 1;
        }
    }

    None
}

//recursive implementation

pub fn recursive_bin_search<T>(haystack : &[T], needle : T, low: usize, high : usize) -> Option<usize>
    where T : Copy + PartialOrd + PartialEq
{
    while low <= high {

        let mid = low + (high - low) /2;

        if needle == haystack[mid] {
            return Some(mid);
        }else if needle < haystack[mid] {
            return recursive_bin_search(haystack, needle, low , mid-1);
        }else{
            return recursive_bin_search(haystack, needle, mid+1, high);
        }
    }
    None
}