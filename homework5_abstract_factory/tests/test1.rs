#[path = "../src/bin.rs"]
mod sorting;

#[cfg(test)]
mod tests_one {
    use super::sorting;
    use lib_sorting::*;
    use sorting::*;
    #[test]
    fn test_merge_sort() {
        let vec0: Vec<i32> = vec![0, 3, 54, -1, 3, 5, 7, 7, 10, 4, 2321, -123, 3];
        let factory_merge = MergeSortFactory;
        let mut merge_instance = factory_merge.instantiate_sorting();
        merge_instance.assign_data(&vec0);
        merge_instance.do_task();
        let answer = vec![-123, -1, 0, 3, 3, 3, 4, 5, 7, 7, 10, 54, 2321];
        assert!(merge_instance.validate_data(&answer).is_some());
    }
    #[test]
    fn test_quick_sort() {
        let vec0: Vec<i32> = vec![0, 3, 54, -1, 3, 5, 7, 7, 10, 4, 2321, -123, 3];
        let quick_sort_factory = QuickSortFactory;
        let mut quick_instance = quick_sort_factory.instantiate_sorting();
        quick_instance.assign_data(&vec0);
        quick_instance.do_task();
        let answer = vec![-123, -1, 0, 3, 3, 3, 4, 5, 7, 7, 10, 54, 2321];
        assert!(quick_instance.validate_data(&answer).is_some());
    }
    #[test]
    fn test_bubble_sort() {
        let vec0: Vec<i32> = vec![0, 3, 54, -1, 3, 5, 7, 7, 10, 4, 2321, -123, 3];
        let bubble_factory = QuickSortFactory;
        let mut bubble_instance = bubble_factory.instantiate_sorting();
        bubble_instance.assign_data(&vec0);
        bubble_instance.do_task();
        let answer = vec![-123, -1, 0, 3, 3, 3, 4, 5, 7, 7, 10, 54, 2321];
        assert!(bubble_instance.validate_data(&answer).is_some());
    }
    #[test]
    fn test_perform() {
        let vec0: Vec<i32> = vec![0, 3, 54, -1, 3, 5, 7, 7, 10, 4, 2321, -123, 3];
        let factory_merge = MergeSortFactory;
        let mut merge_instance = factory_merge.instantiate_sorting();
        merge_instance.perform(&vec0);
        let answer = vec![-123, -1, 0, 3, 3, 3, 4, 5, 7, 7, 10, 54, 2321];
        assert!(merge_instance.validate_data(&answer).is_some());
    }
}
