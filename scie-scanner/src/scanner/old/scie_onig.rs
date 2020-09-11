//
//
use crate::scanner::old::scie_error::ScieOnigError;
use onig::{EncodedChars, Syntax, Region, EncodedBytes, SearchOptions, MatchParam};
use std::ptr::null_mut;
use std::sync::Mutex;
use std::os::raw::c_int;
use onig_sys::OnigRegSetStruct;

lazy_static! {
    static ref REGEX_NEW_MUTEX: Mutex<()> = Mutex::new(());
}

bitflags! {
    pub struct ScieOnigOptions: onig_sys::OnigOptionType {
        const REGEX_OPTION_NONE
            = onig_sys::ONIG_OPTION_NONE;
    }
}

pub struct ScieOnig {
    raw: onig_sys::OnigRegex,
}

impl ScieOnig {
    pub fn demo_new(pattern: &str) -> Result<Self, ScieOnigError> {
        let option = ScieOnigOptions::REGEX_OPTION_NONE;
        let syntax = Syntax::default();

        let mut reg: onig_sys::OnigRegex = null_mut();
        let reg_ptr = &mut reg as *mut onig_sys::OnigRegex;

        // We can use this later to get an error message to pass back
        // if regex creation fails.
        let mut error = onig_sys::OnigErrorInfo {
            enc: null_mut(),
            par: null_mut(),
            par_end: null_mut(),
        };

        let err = unsafe {
            // Grab a lock to make sure that `onig_new` isn't called by
            // more than one thread at a time.
            let _guard = REGEX_NEW_MUTEX.lock().unwrap();
            onig_sys::onig_new(
                reg_ptr,
                pattern.start_ptr(),
                pattern.limit_ptr(),
                option.bits(),
                pattern.encoding(),
                syntax as *const Syntax as *mut Syntax as *mut onig_sys::OnigSyntaxType,
                &mut error,
            )
        };

        if err == onig_sys::ONIG_NORMAL as i32 {
            Ok(ScieOnig { raw: reg })
        } else {
            Err(ScieOnigError::from_code_and_info(err, &error))
        }
    }

    pub fn match_str(&self) {
        let at = 0;
        let chars = EncodedBytes::ascii(b"a");
        let match_param = MatchParam::default();
        let options = SearchOptions::SEARCH_OPTION_NONE;
        let region: Option<&mut Region> = None;

        let r = unsafe {
            let offset = chars.start_ptr().add(at);
            assert!(offset <= chars.limit_ptr());
            onig_sys::onig_match_with_param(
                self.raw,
                chars.start_ptr(),
                chars.limit_ptr(),
                offset,
                match region {
                    Some(region) => region as *mut Region as *mut onig_sys::OnigRegion,
                    None => std::ptr::null_mut(),
                },
                options.bits(),
                match_param.as_raw(),
            )
        };

        println!("{:?}", r);
    }

    pub fn search_str(&self) {
        let at = 0;
        let chars = EncodedBytes::ascii(b"azzzzzzz");
        let match_param = MatchParam::default();
        let options = SearchOptions::SEARCH_OPTION_NONE;
        let region: Option<&mut Region> = None;
        let from = 5;
        let to = 10;
        let mut pos: c_int = 0;


        let (beg, end) = (chars.start_ptr(), chars.limit_ptr());
        let r = unsafe {
            let start = beg.add(from);
            let range = beg.add(to);

            let mut some = OnigRegSetStruct {
                _unused: []
            };
            onig_sys::onig_regset_search(
                &mut some,
                beg,
                end,
                start,
                &0,
                0,
                0,
                &mut pos,
            );
        };

        println!("match_param: {:?}", match_param.as_raw());

        println!("{:?}", r);
    }

    pub fn create_onig_scanner(_sources: Vec<String>) {}
}

#[cfg(test)]
mod tests {
    use crate::scanner::old::scie_onig::ScieOnig;

    #[test]
    fn it_works() {
        let onig = ScieOnig::demo_new(r"\w").unwrap();
        onig.match_str();
        onig.search_str();
        assert!(true)
    }
}
