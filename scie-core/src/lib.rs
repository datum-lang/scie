#[macro_use]
extern crate serde;

extern crate serde_derive;

pub mod analyser;
pub mod identify;
pub mod sima;

#[cfg(test)]
mod tests {
    use crate::analyser::Analyser;
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

        let mut c = Cursor::new(Vec::new());

        let mut line = 1;
        let mut pos = 0;
        let mut index = 0;
        for element in code_file.elements {
            if line < element.line_num {
                c.write_all("\n".as_ref()).unwrap();
                line = line + 1;
                continue;
            }

            println!("{:?}, {:?}", element.start_index, pos);
            if element.start_index == pos {
                c.write_all(element.value.as_ref()).unwrap();
                pos = pos + element.end_index;
            } else {
                c.write_all(" ".as_ref()).unwrap();
            }
            index = index + 1;
        }

        c.seek(SeekFrom::Start(0)).unwrap();

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
