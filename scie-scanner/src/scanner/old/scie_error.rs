// > The MIT License (MIT)
// >
// > Copyright (c) 2015 Will Speak <will@willspeak.me>, Ivan Ivashchenko
// > <defuz@me.com>, and contributors.
// >
// > Permission is hereby granted, free of charge, to any person obtaining a copy
// > of this software and associated documentation files (the "Software"), to deal
// > in the Software without restriction, including without limitation the rights
// > to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// > copies of the Software, and to permit persons to whom the Software is
// > furnished to do so, subject to the following conditions:
// >
// > The above copyright notice and this permission notice shall be included in all
// > copies or substantial portions of the Software.
// >
// > THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// > IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// > FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// > AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// > LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// > OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// > SOFTWARE.


use std::os::raw::c_int;
use std::{error, fmt, str};

/// This struture represents an error from the underlying Oniguruma libray.
pub struct ScieOnigError {
    code: c_int,
    description: String,
}

impl ScieOnigError {
    pub fn from_code_and_info(code: c_int, info: &onig_sys::OnigErrorInfo) -> ScieOnigError {
        ScieOnigError::new(code, info)
    }

    // fn from_code(code: c_int) -> ScieOnigError {
    //     ScieOnigError::new(code, null())
    // }

    fn new(code: c_int, info: *const onig_sys::OnigErrorInfo) -> ScieOnigError {
        let buff = &mut [0; onig_sys::ONIG_MAX_ERROR_MESSAGE_LEN as usize];
        let len = unsafe { onig_sys::onig_error_code_to_str(buff.as_mut_ptr(), code, info) };
        let description = str::from_utf8(&buff[..len as usize]).unwrap();
        ScieOnigError {
            code,
            description: description.to_owned(),
        }
    }

    /// Return Oniguruma engine error code.
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Return error description provided by Oniguruma engine.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl error::Error for ScieOnigError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for ScieOnigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oniguruma error: {}", self.description())
    }
}

impl fmt::Debug for ScieOnigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error({}, {})", self.code, self.description())
    }
}
