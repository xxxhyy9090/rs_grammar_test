#[cfg(test)]
mod tests {
    use crate::sort::{bubble_sort,
                      selection_sort,
                      get_sort_fn,
                      parse_sort_mod,
                      insertion_sort,
                      merge_sort::merge_sort,
                      quick_sort::{quick_sort,
                                  partition}};

    #[test]

    fn bubble_sort_test() {
        let mut bubble_sort_num_tests:Vec<i32> = Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let bubble_sort_nums_tests_right:Vec<i32> = Vec::from([10,21,23,30,45,50,54,70,90]);
        bubble_sort(&mut bubble_sort_num_tests);
        assert_eq!(bubble_sort_num_tests,bubble_sort_nums_tests_right);
    }
    #[test]
    fn selection_sort_test() {
        let mut selection_sort_num_tests:Vec<i32> = Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let selection_sort_num_right:Vec<i32> = Vec::from([10,21,23,30,45,50,54,70,90]);
        selection_sort(&mut selection_sort_num_tests);
        assert_eq!(selection_sort_num_tests,selection_sort_num_right)
    }
    #[test]
    fn insertion_sort_test(){
        let mut insertion_sort_num_tests:Vec<i32> =Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let insertion_sort_num_right=Vec::from([10,21,23,30,45,50,54,70,90]);
        insertion_sort(&mut insertion_sort_num_tests);
        assert_eq!(insertion_sort_num_tests,insertion_sort_num_right);
    }
    #[test]
    fn merge_sort_test(){
        let mut merge_sort_test=Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let merge_sort_right=Vec::from([10,21,23,30,45,50,54,70,90]);
        merge_sort(&mut merge_sort_test);
        assert_eq!(merge_sort_test,merge_sort_right);
    }
    #[test]
    fn quick_sort_test(){
        let mut quick_sort_test=Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let quic_sort_right=Vec::from([10,21,23,30,45,50,54,70,90]);
        quick_sort(&mut quick_sort_test);
        assert_eq!(quick_sort_test,quic_sort_right);
    }
    #[test]
    fn partition_test(){
        let mut partition_test=Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let partition_num=partition(&mut partition_test);

        assert_eq!(partition_num,2);
        assert_eq!(partition_test[2],23);
        for i in 0..partition_num {
            assert!(partition_test[i]<23);
        }
        for i in partition_num+1..partition_test.len() {
            assert!(partition_test[i]>23);
        }
    }
    #[test]
    fn get_sort_fn_test(){
        let sort_mod_test_bubble=parse_sort_mod("bubble").unwrap();
        let sort_mod_test_selection=parse_sort_mod("selection").unwrap();
        let sort_mod_test_insertion=parse_sort_mod("insertion").unwrap();
        let sort_mod_test_merge_sort=parse_sort_mod("merge").unwrap();
        let sort_mod_test_quicksort=parse_sort_mod("quick").unwrap();
        let bubble_test=get_sort_fn(sort_mod_test_bubble);
        let selection_test=get_sort_fn(sort_mod_test_selection);
        let insertion_test=get_sort_fn(sort_mod_test_insertion);
        let merge_test=get_sort_fn(sort_mod_test_merge_sort);
        let quicksort_test=get_sort_fn(sort_mod_test_quicksort);

        let num_clone=Vec::from([10, 21, 30, 50, 45, 90, 70, 54, 23]);
        let right_clone=Vec::from([10,21,23,30,45,50,54,70,90]);

        let mut test_num_insertion=num_clone.clone();
        let mut test_num_bubble= num_clone.clone();
        let mut test_num_selection= num_clone.clone();
        let mut test_num_merge= num_clone.clone();
        let mut test_num_quick= num_clone.clone();
        let test_num_right=right_clone.clone();
        bubble_test(&mut test_num_bubble);
        selection_test(&mut test_num_selection);
        insertion_test(&mut test_num_insertion);
        merge_test(&mut test_num_merge);
        quicksort_test(&mut test_num_quick);
        assert_eq!(test_num_right,test_num_bubble);
        assert_eq!(test_num_right,test_num_selection);
        assert_eq!(test_num_right,test_num_insertion);
        assert_eq!(test_num_right,test_num_merge);
        assert_eq!(test_num_right,test_num_quick);
    }
}
