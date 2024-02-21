#[cfg(test)]
mod test{
    use crate::exponential_search::exponen_search;
    use crate::binary_search::*;

    //passing tests
    #[test]
    fn recursive_binary_search() {
        let haystack = ['a', 'b', 'c' , 'd'];
        let needle = 'c';

        assert_eq!(Some(2), recursive_bin_search(&haystack, needle, 0, haystack.len()-1));
    }
    #[test]
    fn iterative_binary_search() {
        let haystack = ['a', 'b', 'c' , 'd'];
        let needle = 'c';

        assert_eq!(Some(2), recursive_bin_search(&haystack, needle, 0, haystack.len()-1));
    }
    #[test]
    fn exponential_search() {
        let haystack = ['a', 'b', 'c' , 'd'];
        let needle = 'c';

        assert_eq!(Some(2), exponen_search(&haystack, needle));
    }

    //failing tests
    #[test]
    fn f_recursive_binary_search() {
        let haystack = ['a', 'b', 'c' , 'd'];
        let needle = 'e';

        //needle absent
        assert_eq!(Some(2), recursive_bin_search(&haystack, needle, 0, haystack.len()-1));
    }
    #[test]
    fn f_iterative_binary_search() {
        let haystack = ['a', 'b', 'c' , 'd'];
        let needle = 'c';

        //index out of bounds
        assert_eq!(Some(4), recursive_bin_search(&haystack, needle, 0, haystack.len()-1));
    }
    #[test]
    fn f_exponential_search() {
        let haystack = ['a', 'b', 'c' , 'd'];
        let needle = 'c';

        //incorrect comparison index
        assert_eq!(Some(0), exponen_search(&haystack, needle));
    }

}
