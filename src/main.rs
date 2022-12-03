mod accessor;
mod aligner;
mod filemanager;
mod line;
use structopt::StructOpt;

#[derive(StructOpt, Clone)]
#[structopt(name = "alcom-rs", about = "Powerful aligner of assembler comments")]
pub struct Arguments {
    #[structopt(
        short("e"),
        long,
        help = "If setted we will place the separator on each line"
    )]
    place_separator_on_each_line: bool,

    #[structopt(
        short,
        long,
        help = "Set separator(devider) between code and comments",
        default_value = ";"
    )]
    separator: char,

    #[structopt(
        short,
        long,
        help = "Set amount of spaces between code and separator",
        default_value = "4"
    )]
    lmargin: u8,

    #[structopt(
        short,
        long,
        help = "Set amount of spaces between separator and comment",
        default_value = "4"
    )]
    rmargin: u8,

    #[structopt(name = "Files' paths to align", parse(from_os_str))]
    files: Vec<std::path::PathBuf>,
}

fn main() {
    let args = Arguments::from_args();
    let aligner = aligner::Aligner::new();
    aligner.align(&args);
}
