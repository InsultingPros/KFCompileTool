pub mod conflict_resolver;
pub mod io;
pub mod zip_extension;

use crate::constants::LINE_SEPARATOR;

pub fn print_fancy_block(input: &str) {
    println!("{LINE_SEPARATOR}{input}{LINE_SEPARATOR}");
}
