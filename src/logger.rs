use std::{fs,io::Write,path::Path};
pub const MAX_LOG: u64=256*1024;
pub fn append(dir:&Path,message:&str){let _=fs::create_dir_all(dir);let log=dir.join("log.txt");if fs::metadata(&log).map(|m|m.len()>MAX_LOG).unwrap_or(false){let _=fs::remove_file(dir.join("log.old.txt"));let _=fs::rename(&log,dir.join("log.old.txt"));}if let Ok(mut f)=fs::OpenOptions::new().create(true).append(true).open(log){let _=writeln!(f,"{message}");}}
