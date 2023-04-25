use lib_sorting::{CpuBoundTask, Sorting};
mod quicksort;
use quicksort::method_impl::sort as quicksort;
mod mergesort;
use mergesort::method_implement::sort as mergesirt;
mod bubblesort;
use bubblesort::method_impl::sort as bubblesort;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;
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
    #[clap(value_enum,short, long, value_parser, verbatim_doc_comment,default_value_t = SortMethod::Quick)]
    method: SortMethod,
}
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SortMethod {
    Bubble,
    Merge,
    Quick,
}
impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SortMethod::Bubble => write!(f, "Bubble"),
            SortMethod::Merge => write!(f, "Merge"),
            SortMethod::Quick => write!(f, "Quick"),
        }
    }
}
//-------------------------------------------------------
//-------------------------------------------------------

fn main() -> std::io::Result<()> {
    let mut root_d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    println!("root is {}", root_d.display());
    let mut buffer = vec![];
    let mut val: i32 = -1_000_000;
    for _i in 0..=9 {
        println!("writing {}", val);
        buffer.write_i32::<LittleEndian>(val).unwrap();
        val -= 1;
    }
    let mut f = File::create("foo.txt")?;
    f.write_all(&buffer)?;
    std::mem::drop(f); // dropping
    let mut f = File::open("foo.txt")?;
    for _i in 0..=9 {
        let readed = f.read_i32::<LittleEndian>().unwrap();
        println!("readed:{}", readed);
    }
    Ok(())
    //     let args = Args::parse();
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
