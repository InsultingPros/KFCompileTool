pub mod conflict_resolver;
pub mod io;

const LINE_SEPARATOR: &str = "\n######################################################\n";

pub fn print_fancy_block(input: &str) {
    println!("{LINE_SEPARATOR}{input}{LINE_SEPARATOR}");
}
