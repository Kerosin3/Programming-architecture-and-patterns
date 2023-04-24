use lib_sorting::{CpuBoundTask, Sorting};
mod quicksort;
use quicksort::method_impl::sort as quicksort;
mod mergesort;
use mergesort::method_implement::sort as mergesirt;
mod bubblesort;
use bubblesort::method_impl::sort as bubblesort;
use clap::Parser;
use std::path::PathBuf;
//-------------------------------------------------------
//-------------------------------------------------------
#[derive(Debug, clap::Parser, Clone)]
#[clap(long_about = "The worst Hello World!!! App in the world!")]
struct Args {
    /// input filename
    /// please specify input filename
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    input_filename: String,
    /// method
    /// please specity sorting method
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    method: SortMethod,
}
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SortMethod {
    Bubble,
    Merge,
    Quick,
}
//-------------------------------------------------------
//-------------------------------------------------------

fn main() {
    let args = Args::parse();
}
#[derive(Default, Debug)]
pub struct BubbleSort {
    vec: Vec<i32>,
}
#[derive(Default, Debug)]
pub struct MergeSort {
    vec: Vec<i32>,
}
#[derive(Default, Debug)]
pub struct QuickSort {
    vec: Vec<i32>,
}
// concrete factories
pub struct BubbleSortFactory;
pub struct MergeSortFactory;
pub struct QuickSortFactory;

impl Sorting for BubbleSort {
    fn do_task(&mut self) {
        println!("sorting with bubblesort method!");
        bubblesort(&mut self.vec);
    }
    fn assign_data(&mut self, vec: &[i32]) {
        self.vec = vec.to_owned();
    }
}
impl Sorting for MergeSort {
    fn do_task(&mut self) {
        println!("sorting with mergesort method!");
        mergesirt(&mut self.vec)
    }
    fn assign_data(&mut self, vec: &[i32]) {
        self.vec = vec.to_owned();
    }
}
impl Sorting for QuickSort {
    fn do_task(&mut self) {
        println!("sorting with quicksort method!");
        quicksort(&mut self.vec)
    }
    fn assign_data(&mut self, vec: &[i32]) {
        self.vec = vec.to_owned();
    }
}

impl CpuBoundTask<BubbleSort> for BubbleSortFactory {
    fn instantiate_sorting(&self) -> BubbleSort {
        BubbleSort::default()
    }
}

impl CpuBoundTask<MergeSort> for MergeSortFactory {
    fn instantiate_sorting(&self) -> MergeSort {
        MergeSort::default()
    }
}
impl CpuBoundTask<QuickSort> for QuickSortFactory {
    fn instantiate_sorting(&self) -> QuickSort {
        QuickSort::default()
    }
}
