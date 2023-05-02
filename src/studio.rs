// #[cfg(target_os = "windows")]
// use crate::credentials;

use keyring;
use rbx_cookie;

static STUDIO_AUTH_URL: &str = "https://www.roblox.com:RobloxStudioAuth";

#[cfg(target_os = "windows")]
pub fn get_auth_credential() {
    rbx_cookie::get();
}

#[cfg(target_os = "windows")]
pub fn set_auth_credential(token_name: &str, token_value: &str) -> keyring::Result<()> {
    let target = &format!("{}{}", STUDIO_AUTH_URL, token_name);
    let entry = keyring::Entry::new_with_target(target, "", "")?;

    entry.set_password(token_value)?;

    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn set() {
    None
}
