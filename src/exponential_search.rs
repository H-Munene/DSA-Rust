use std::cmp::min;
use crate::binary_search::recursive_bin_search;

//
pub fn exponen_search<T>(haystack:&[T], needle: T) -> Option<usize>
where T: Copy + Ord
{
    let elements = haystack.len() ;
    let mut index = 1;

    if haystack[0] == needle {
        return Some(0);
    }else {
        while index < elements && haystack[index] <= needle {
            //{1,2,3,4,5} needle 5
            index *= 2;
        }
        //makes recursive call to already defined recursive binary search function
        return recursive_bin_search(&haystack, needle, index /2 , min(index, elements));
    }
}


/*
 //{1,2,3,4,5,6} needle 5
l = 6
i = 1 true -> i <  l , a[i] <= t
 i = 2 true -> i < l , a[i] <= t
 i = 4 true -> i < n (4 < 5 ) , a[i] <= t
 i = 8 f -> i < n (8 < 5) , a[i] <= t
  case f
    recur_bin_search(a, i/2(4), min(i(8) , l(6)), t)
 */
