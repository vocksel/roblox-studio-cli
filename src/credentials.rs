/* MIT License

Copyright (c) 2021 Blake Mealey

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. */

// Implementation heavily based on https://github.com/rust-lang/cargo/blob/master/crates/credential/cargo-credential-wincred/src/main.rs

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{FILETIME, TRUE};
use winapi::um::wincred;

type Error = Box<dyn std::error::Error>;

/// Converts a string to a nul-terminated wide UTF-16 byte sequence.
fn wstr(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
    if wide.iter().any(|b| *b == 0) {
        panic!("nul byte in wide string");
    }
    wide.push(0);
    wide
}

pub fn get(target_name: &str) -> Result<String, Error> {
    let target_name = wstr(target_name);
    let mut p_credential: wincred::PCREDENTIALW = std::ptr::null_mut();
    unsafe {
        if wincred::CredReadW(
            target_name.as_ptr(),
            wincred::CRED_TYPE_GENERIC,
            0,
            &mut p_credential,
        ) != TRUE
        {
            return Err(
                format!("failed to fetch token: {}", std::io::Error::last_os_error()).into(),
            );
        }
        let bytes = std::slice::from_raw_parts(
            (*p_credential).CredentialBlob,
            (*p_credential).CredentialBlobSize as usize,
        );
        String::from_utf8(bytes.to_vec()).map_err(|_| "failed to convert token to UTF8".into())
    }
}

pub fn set(target_name: &str, token_value: &str) -> Result<bool, Error> {
    unsafe {
        let mut credential = wincred::CREDENTIALW {
            Flags: 0,
            Type: 0,
            TargetName: wstr(target_name).as_mut_ptr(),
            Comment: std::ptr::null_mut(),
            LastWritten: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            CredentialBlobSize: 0,
            CredentialBlob: token_value.to_owned().as_mut_ptr(),
            Persist: wincred::CRED_TYPE_DOMAIN_PASSWORD,
            AttributeCount: 0,
            Attributes: &mut wincred::CREDENTIAL_ATTRIBUTEW {
                Flags: 0,
                Keyword: std::ptr::null_mut(),
                Value: std::ptr::null_mut(),
                ValueSize: 0,
            },
            TargetAlias: std::ptr::null_mut(),
            UserName: std::ptr::null_mut(),
        };

        let credential_ptr = std::ptr::addr_of_mut!(credential);

        if wincred::CredWriteW(credential_ptr, 0) != TRUE {
            return Err(format!("failed to set token: {}", std::io::Error::last_os_error()).into());
        } else {
            println!("hopefully stored the credential?");
            return Ok(true);
        }
    }
}
