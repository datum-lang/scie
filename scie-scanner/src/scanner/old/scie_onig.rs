//
//
use crate::scanner::old::scie_error::ScieOnigError;
use onig::{EncodedChars, Syntax};
use std::ptr::null_mut;
use std::sync::Mutex;

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

        // `onig_new`.
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
    pub fn create_onig_scanner(_sources: Vec<String>) {}
}

#[cfg(test)]
mod tests {
    use crate::scanner::old::scie_onig::ScieOnig;

    #[test]
    fn it_works() {
        let _result = ScieOnig::demo_new(r"^");
        assert!(true)
    }
}
