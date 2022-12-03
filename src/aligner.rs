use std::path::PathBuf;

use crate::{accessor::FileAccessor, filemanager::FileManager, line::Line, Arguments};

pub struct Aligner {
    filemanager: FileManager,
}

impl Aligner {
    pub fn new() -> Aligner {
        Aligner {
            filemanager: FileManager::default(),
        }
    }

    pub fn align(&self, args: &Arguments) {
        for file in args.files.iter() {
            self.align_file(file, args);
        }
    }

    pub fn align_file(&self, file: &PathBuf, args: &Arguments) {
        let tempfile = match self.filemanager.create_temporary_file() {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let lines = match FileAccessor::read(file, args.separator) {
            Ok(l) => l,
            Err(e) => {
                println!(
                    "Failed to align file: {}! {}",
                    file.to_str().unwrap_or(""),
                    e
                );
                return;
            }
        };

        if lines.is_empty() {
            return;
        } // File is empty

        let longest = lines.iter().max().unwrap().code().len();

        let lines_ = lines
            .iter()
            .map(|line| self.prettify(line, args, longest))
            .collect::<Vec<String>>();

        FileAccessor::write(&tempfile, lines_).unwrap();
        FileManager::rename(&tempfile, file).ok();
    }

    pub fn prettify(&self, line: &Line, args: &Arguments, longest: usize) -> String {
        if line.has_code() && line.has_comment() {
            Self::prettify_code_and_comment(line, args, longest)
        } else if line.has_code() && !line.has_comment() {
            Self::prettify_code(line, args, longest)
        } else if !line.has_code() && line.has_comment() {
            Self::prettify_comment(line, args, longest)
        } else {
            Self::prettify_empty(args, longest)
        }
    }

    fn prettify_code_and_comment(line: &Line, args: &Arguments, longest: usize) -> String {
        let mut s = line.code().clone();

        s += &" ".repeat(longest - s.len() + args.lmargin as usize);
        s += &format!("{}", args.separator);
        s += &" ".repeat(args.rmargin.into());
        s += line.comment();
        s
    }

    fn prettify_code(line: &Line, args: &Arguments, longest: usize) -> String {
        let mut s = line.code().clone();

        if args.place_separator_on_each_line {
            s += &" ".repeat(longest - s.len() + args.lmargin as usize);
            s += &format!("{}", args.separator);
        }
        s
    }

    fn prettify_comment(line: &Line, args: &Arguments, longest: usize) -> String {
        // TODO args
        /*
         -d, --disable-fix-of-long-comments            Disables fixing long comment lines
         -f, --place-long-comments-on-separate-line    Long comments are splitted and will be placed on new lines without code
        */
        return " ".repeat(longest + args.rmargin as usize)
            + ";"
            + &" ".repeat(args.lmargin as usize)
            + line.comment();
    }

    fn prettify_empty(args: &Arguments, longest: usize) -> String {
        if args.place_separator_on_each_line {
            " ".repeat(longest + args.rmargin as usize) + ";"
        } else {
            "".into()
        }
    }
}
