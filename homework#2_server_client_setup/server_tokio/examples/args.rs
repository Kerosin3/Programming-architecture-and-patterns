#![allow(dead_code)]
#![allow(unused_imports)]
use clap::{Args, Parser, Subcommand};
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ClientArgs {
    /// specify a valid device name and get current info
    #[arg(short = 'd')]
    pub devname: String,
    /// specify if you want to enable or disable the device
    #[arg(short = 'e')]
    pub enable: Option<bool>,
}
fn main() {} // ????

/*
 *
 *
 *
 *
#[clap(subcommand)]
#[derive(Debug,Subcommand)]
pub enum EnableEnum{
    Enable,
    Disabe,
    NotSet,
}*/
