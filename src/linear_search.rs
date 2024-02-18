
pub fn lin_search<T>(haystack: &[T], needle: T ) -> Option<i32>
    where T: Copy + Clone + PartialEq
{
    let mut count = 0;
    for straw in haystack.iter() {
        if &needle == straw {
            return Some(count);
        }
        count +=1;
    }

    None
}