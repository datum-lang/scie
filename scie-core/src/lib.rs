#[macro_use]
extern crate serde;

extern crate serde_derive;

use scie_model::artifact::CodeFile;
use std::io::{Cursor, Seek, SeekFrom, Write};

pub mod analyser;
pub mod identify;
pub mod sima;

pub fn code_to_file(code_file: &CodeFile) -> Cursor<Vec<u8>> {
    let mut c = Cursor::new(Vec::new());

    let mut line = 1;
    let mut pos = 0;
    let mut index = 0;

    let mut stop = false;
    let length = code_file.elements.len();

    while !stop {
        let element = &code_file.elements[index];
        if line < element.line_num {
            c.write_all("\n".as_ref()).unwrap();
            line = line + 1;
            continue;
        }

        if element.start_index == pos {
            c.write_all(element.value.as_ref()).unwrap();
            pos = element.end_index;
        } else {
            let has_next = index < length - 1;
            if has_next {
                let next_element = &code_file.elements[index + 1];
                let offset = next_element.end_index - pos as i32;
                c.write_all(" ".repeat(offset as usize).as_ref()).unwrap();
            }
        }
        if index + 1 == length {
            stop = true;
        }
        index = index + 1;
    }

    c.seek(SeekFrom::Start(0)).unwrap();
    c
}

#[cfg(test)]
mod tests {
    use crate::analyser::Analyser;
    use crate::code_to_file;
    use scie_model::artifact::CodeFile;
    use std::fs::File;
    use std::io::{Cursor, Read, Seek, SeekFrom, Write};
    use std::path::PathBuf;

    #[test]
    fn should_build_from_element() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        let lang = root_dir
            .clone()
            .parent()
            .unwrap()
            .join("fixtures")
            .join("projects")
            .join("java")
            .join("simple")
            .join("settings.gradle");

        let files = Analyser::ident_by_dir(&lang, false, false);
        let code_file = files[0].clone();

        let mut c = code_to_file(&code_file);

        output_to_file(&mut c);
    }

    #[test]
    fn should_build_fake_file() {
        let mut c = Cursor::new(Vec::new());

        let demo = "Demo";

        c.write_all(demo.as_ref()).unwrap();
        write_new_line(&mut c);

        c.seek(SeekFrom::Start(0)).unwrap();

        output_to_file(&mut c);
    }

    fn write_new_line(c: &mut Cursor<Vec<u8>>) -> usize {
        // \n in ascii is 10
        c.write(&[10]).unwrap()
    }

    fn output_to_file(c: &mut Cursor<Vec<u8>>) {
        let mut file = File::create("foo.txt").unwrap();

        let mut out = Vec::new();
        c.read_to_end(&mut out).unwrap();
        file.write_all(&*out).unwrap();
    }

    fn output_to_vec(c: &mut Cursor<Vec<u8>>) {
        let mut out = Vec::new();
        c.read_to_end(&mut out).unwrap();
        println!("{:?}", out);
    }
}
