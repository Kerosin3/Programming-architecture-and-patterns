#[path = "../src/bin.rs"]
mod sorting;

#[cfg(test)]
mod tests_one {
    use super::sorting;
    use lib_sorting::*;
    use sorting::*;
    #[test]
    fn it_works() {
        let vec0: Vec<i32> = vec![0, 3, 54, -1, 3, 5, 7, 7, 10, 4, 2321, -123, 3];
        let a = MergeSortFactory;
        let b = BubbleSortFactory;
        let mut aa = a.instantiate_sorting();
        let mut bb = b.instantiate_sorting();
        aa.assign_data(&vec0);
        bb.assign_data(&vec0);
        println!("data is {:?}", aa);
        println!("data is {:?}", bb);
        aa.do_task();
        bb.do_task();
        println!("data is {:?}", aa);
        println!("data is {:?}", bb);
    }
}
