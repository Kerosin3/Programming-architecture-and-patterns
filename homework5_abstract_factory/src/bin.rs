use lib_sorting::{CpuBoundTask, Sorting};
mod quicksort;
use quicksort::method_impl::sort as quicksort;
mod mergesort;
use mergesort::method_implement::sort as mergesirt;
mod bubblesort;

use bubblesort::method_impl::sort as bubblesort;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use thiserror::Error;
//-------------------------------------------------------
//-------------------------------------------------------
#[derive(Debug, clap::Parser, Clone)]
#[clap(long_about = "Abstrac factory applied to sorting example")]
struct Args {
    /// input filename
    /// please specify input filename
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    input_filename: Option<String>,
    /// output filename
    /// please specify output filename
    #[clap(short, long, value_parser, verbatim_doc_comment)]
    output_filename: Option<String>,
    /// method
    /// please specify sorting method
    #[clap(value_enum,short, long, value_parser, verbatim_doc_comment,default_value_t = SortMethod::Quick)]
    method: SortMethod,
    /// method
    /// please specify app workmode
    #[clap(value_enum, short, long, value_parser, verbatim_doc_comment)]
    workmode: AppWorkmode,
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
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AppWorkmode {
    WriteRandom,
    ReadFile,
    Operating,
}

//-------------------------------------------------------
//-------------------------------------------------------
const N_ELEMENT: usize = 10;
//-------------------------------------------------------
//-------------------------------------------------------
//generate random numbers
use std::iter::repeat_with;
fn generate_random() -> Vec<i32> {
    let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(N_ELEMENT).collect();
    v
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // parse workmode
    let work_mode = match args.workmode {
        AppWorkmode::WriteRandom => {
            println!(
                "Writing random i32 to file [{}] n= [{}]]",
                args.output_filename
                    .to_owned()
                    .ok_or::<anyhow::Error>(AppError::ErrorProcessingOutputFIle.into())?,
                N_ELEMENT
            );
            AppWorkmode::WriteRandom
        }
        AppWorkmode::ReadFile => {
            println!(
                "Reading i32 from file [{}] n= [{}]]",
                args.input_filename
                    .to_owned()
                    .ok_or::<anyhow::Error>(AppError::ErrorProcessingInputFile.into())?,
                N_ELEMENT
            );
            AppWorkmode::ReadFile
        }
        AppWorkmode::Operating => {
            println!(
                "Writing random i32 to file [{}], n= [{}],\n
                    Reading i32 from file [{}] n= [{}]]",
                args.output_filename
                    .to_owned()
                    .ok_or::<anyhow::Error>(AppError::ErrorProcessingOutputFIle.into())?,
                N_ELEMENT,
                args.input_filename
                    .to_owned()
                    .ok_or::<anyhow::Error>(AppError::ErrorProcessingInputFile.into())?,
                N_ELEMENT
            );
            AppWorkmode::Operating
        }
    };
    // preform wormode
    match work_mode {
        /*writes 50 random i32 to a file*/
        AppWorkmode::WriteRandom => {
            let filen =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(args.output_filename.unwrap());
            let mut file_w = File::create(filen)?;
            for item in generate_random().iter() {
                println!("writing {}", *item);
                file_w.write_i32::<LittleEndian>(*item).unwrap(); // write to buffer
            }
        }
        /*read 50 random i32 from a file*/
        AppWorkmode::ReadFile => {
            let filen =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(args.input_filename.unwrap());

            let mut f = File::open(filen)?;
            let mut buf_readed: Vec<i32> = vec![];
            for _i in 0..N_ELEMENT {
                let readed = f.read_i32::<LittleEndian>().unwrap();
                println!("readed:{}", readed);
                buf_readed.push(readed); // tush to buf
            }
        }
        /*read i32 little endian from file, sorting and writes sorted array to a specified file*/
        AppWorkmode::Operating => {
            let input_filename =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(args.input_filename.unwrap());
            let out_filename =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(args.output_filename.unwrap());
            let mut file_w = File::create(&out_filename)?; // create for writing
            let mut f = File::open(input_filename)?;
            let mut buf_readed: Vec<i32> = vec![];
            for _i in 0..N_ELEMENT {
                let readed = f.read_i32::<LittleEndian>().unwrap();
                println!("readed:{}", readed);
                buf_readed.push(readed); // push to buf
            }
            // sorting methods using factory method
            let sorted = match args.method {
                SortMethod::Bubble => {
                    let bubblesort_factory = BubbleSortFactory;
                    let mut bubble_instance = bubblesort_factory.instantiate_sorting();
                    bubble_instance.perform(&buf_readed)
                }
                SortMethod::Merge => {
                    let mergesort_factory = BubbleSortFactory;
                    let mut mergesort_instance = mergesort_factory.instantiate_sorting();
                    mergesort_instance.perform(&buf_readed)
                }
                SortMethod::Quick => {
                    let quiksort_factory = QuickSortFactory;
                    let mut quick_instance = quiksort_factory.instantiate_sorting();
                    quick_instance.perform(&buf_readed)
                }
            };
            for num in sorted.iter() {
                file_w.write_i32::<LittleEndian>(*num).unwrap(); // write to buffer
            }
        }
    }
    Ok(())
}
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("no input file specified")]
    ErrorProcessingInputFile,
    #[error("no output file specified")]
    ErrorProcessingOutputFIle,
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
// implemet Sorting traits
//##########################################################
impl Sorting for BubbleSort {
    fn do_task(&mut self) {
        println!("sorting with bubblesort method!");
        bubblesort(&mut self.vec);
    }
    fn assign_data(&mut self, vec: &[i32]) {
        self.vec = vec.to_owned();
    }
    fn validate_data(&self, vec: &[i32]) -> Option<()> {
        if vec != self.vec {
            None
        } else {
            Some(())
        }
    }
    fn printout(&self) {
        println!("{:?}", self.vec);
    }
    fn get_data(&self) -> Vec<i32> {
        self.vec.to_owned()
    }
}
//##########################################################
impl Sorting for MergeSort {
    fn do_task(&mut self) {
        println!("sorting with mergesort method!");
        mergesirt(&mut self.vec)
    }
    fn assign_data(&mut self, vec: &[i32]) {
        self.vec = vec.to_owned();
    }
    fn validate_data(&self, vec: &[i32]) -> Option<()> {
        if vec != self.vec {
            None
        } else {
            Some(())
        }
    }
    fn printout(&self) {
        println!("{:?}", self.vec);
    }
    fn get_data(&self) -> Vec<i32> {
        self.vec.to_owned()
    }
}
//##########################################################
impl Sorting for QuickSort {
    fn do_task(&mut self) {
        println!("sorting with quicksort method!");
        quicksort(&mut self.vec)
    }
    fn assign_data(&mut self, vec: &[i32]) {
        self.vec = vec.to_owned();
    }
    fn validate_data(&self, vec: &[i32]) -> Option<()> {
        if vec != self.vec {
            None
        } else {
            Some(())
        }
    }
    fn printout(&self) {
        println!("{:?}", self.vec);
    }
    fn get_data(&self) -> Vec<i32> {
        self.vec.to_owned()
    }
}
//##########################################################
//##########################################################
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
