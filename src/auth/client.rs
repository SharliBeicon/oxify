use std::io;

pub fn finish_login(code: String, secret_id: &str) -> io::Result<(String, String)> {
    Ok(("".into(), "".into()))
}
