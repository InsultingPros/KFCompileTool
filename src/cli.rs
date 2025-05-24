// Reference: https://docs.rs/gumdrop/latest/gumdrop/
/// `kf_compile_tool` supported arguments. For online help check: <https://github.com/InsultingPros/KFCompileTool>
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, gumdrop::Options)]
pub struct MyOptions {
    /// `-h` : print help information.
    #[options(help = "Prints the help message.")]
    pub help: bool,
    #[options(no_short, help = "Waits for user input.")]
    pub hold: bool,
    #[options(free)]
    pub mod_name: Vec<String>,
}
