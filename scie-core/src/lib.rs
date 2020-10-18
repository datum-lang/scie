#[macro_use]
extern crate serde;

extern crate serde_derive;

pub mod analyser;
pub mod identify;
pub mod sima;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Cursor, Read, Seek, SeekFrom, Write};

    #[test]
    fn should_build_fake_file() {
        let mut c = Cursor::new(Vec::new());

        let demo = "Demo";

        c.write_all(demo.as_ref()).unwrap();
        c.write_all("\n".as_ref()).unwrap();

        c.seek(SeekFrom::Start(0)).unwrap();

        output_to_file(&mut c);
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
