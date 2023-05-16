use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef;
use winapi::um::wincred;

type Error = Box<dyn std::error::Error>;

static STUDIO_AUTH_URL: &str = "https://www.roblox.com:RobloxStudioAuth";

/// Converts a string to a nul-terminated wide UTF-16 byte sequence.
fn wstr(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
    if wide.iter().any(|b| *b == 0) {
        panic!("nul byte in wide string");
    }
    wide.push(0);
    wide
}

#[cfg(target_os = "windows")]
pub fn get_auth_credential(token_name: &str) -> Result<String, Error> {
    let target = wstr(&format!("{}{}", STUDIO_AUTH_URL, token_name));
    let mut p_credential: wincred::PCREDENTIALW = std::ptr::null_mut();

    unsafe {
        if wincred::CredReadW(
            target.as_ptr(),
            wincred::CRED_TYPE_GENERIC,
            0,
            &mut p_credential,
        ) != minwindef::TRUE
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

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn get_auth_credential() {
    None
}

#[cfg(target_os = "windows")]
pub fn set_auth_credential(token_name: &str, token_value: &str) -> Result<(), keyring::Error> {
    let target = &format!("{}{}", STUDIO_AUTH_URL, token_name);
    let entry = keyring::Entry::new_with_target(target, "", "")?;

    entry.set_password(token_value)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn set_auth_credential() {
    None
}
