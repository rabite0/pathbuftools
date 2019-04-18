extern crate dirs;

use dirs::home_dir;

use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};

pub trait PathTools {
    fn short_path(&self) -> PathBuf;
    fn short_string(&self) -> String;
    fn name_starts_with(&self, pat: &str) -> bool;
    fn quoted_file_name(&self) -> Option<OsString>;
    fn quoted_path(&self) -> OsString;
}

impl PathTools for Path {
    fn short_path(&self) -> PathBuf {
        if let Some(home) = home_dir() {
            if let Ok(short) = self.strip_prefix(home) {
                let mut path = PathBuf::from("~");
                path.push(short);
                return path
            }
        }
        return self.to_path_buf();
    }

    fn short_string(&self) -> String {
        self.short_path().to_string_lossy().to_string()
    }

    fn name_starts_with(&self, pat: &str) -> bool {
        if let Some(name) = self.file_name() {
            let nbytes = name.as_bytes();
            let pbytes = pat.as_bytes();

            if nbytes.starts_with(pbytes) {
                return true;
            } else {
                return false;
            }
        }
        false
    }

    fn quoted_file_name(&self) -> Option<OsString> {
        if let Some(name) = self.file_name() {
            let mut name = name.as_bytes().to_vec();
            let mut quote = "\"".as_bytes().to_vec();
            let mut quoted = vec![];
            quoted.append(&mut quote.clone());
            quoted.append(&mut name);
            quoted.append(&mut quote);

            let quoted_name = OsStr::from_bytes(&quoted).to_os_string();
            return Some(quoted_name);
        }
        None
    }

    fn quoted_path(&self) -> OsString {
        let mut path = self.to_path_buf().into_os_string().as_bytes().to_vec();
        let mut quote = "\"".as_bytes().to_vec();

        let mut quoted = vec![];
        quoted.append(&mut quote.clone());
        quoted.append(&mut path);
        quoted.append(&mut quote);

        OsString::from_vec(quoted)
    }
}
