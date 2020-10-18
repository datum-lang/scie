use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use scie_model::artifact::CodeFile;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Sima {}

impl Sima {
    pub fn code_to_file(code_file: &CodeFile) -> Cursor<Vec<u8>> {
        let mut c = Cursor::new(Vec::new());

        let mut current_line = 1;
        let mut position = 0;
        let mut index = 0;

        let mut stop = false;
        let length = code_file.elements.len();

        while !stop {
            let element = &code_file.elements[index];
            if current_line < element.line_num {
                c.write_all("\n".as_ref()).unwrap();
                current_line = current_line + 1;
                continue;
            }

            if element.start_index == position {
                c.write_all(element.value.as_ref()).unwrap();
                position = element.end_index;
            } else {
                let has_next = index < length - 1;
                if has_next {
                    let next_element = &code_file.elements[index + 1];
                    let offset = next_element.end_index - position as i32;
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

    fn output_to_file(path: String, c: &mut Cursor<Vec<u8>>) {
        let mut file = File::create(path).unwrap();

        let mut out = Vec::new();
        c.read_to_end(&mut out).unwrap();
        file.write_all(&*out).unwrap();
    }

    fn output_to_vec(c: &mut Cursor<Vec<u8>>) -> Vec<u8> {
        let mut out = Vec::new();
        c.read_to_end(&mut out).unwrap();
        out
    }

    fn write_new_line(c: &mut Cursor<Vec<u8>>) -> usize {
        c.write(&[10]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    use crate::analyser::Analyser;
    use crate::sima::Sima;

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

        let origin_buffer = &mut vec![];
        let mut file = File::open(lang.clone()).unwrap();
        let _result = file.read_to_end(origin_buffer);

        let files = Analyser::ident_by_dir(&lang, false, false);
        let code_file = files[0].clone();
        let mut c = Sima::code_to_file(&code_file);
        let output = Sima::output_to_vec(&mut c);

        assert_eq!(14, output.len());
        assert_eq!(origin_buffer.clone(), output);
    }
}
