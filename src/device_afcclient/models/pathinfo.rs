// "st_blocks": "0",
// "st_mtime": "1744979741182311670",
// "st_ifmt": "S_IFDIR",
// "st_birthtime": "1729274422233349877",
// "st_nlink": "2",
// "st_size": "19136",

use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Directory,
    File,
    Symlink,

    // mostly for jailbroken devices, still not implemented yet
    // TODO: implement afc client for jailbroken devices
    CharDevice,
    BlockDevice,
    NamedPipe,
    Socket,

    Unknown,
}

//['S_IFDIR', 'S_IFCHR', 'S_IFBLK', 'S_IFREG', 'S_IFIFO', 'S_IFLNK', 'S_IFSOCK']

impl From<String> for FileType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "S_IFDIR" => Self::Directory,
            "S_IFREG" => Self::File,
            "S_IFLNK" => Self::Symlink,

            "S_IFCHR" => Self::CharDevice,
            "S_IFBLK" => Self::BlockDevice,
            "S_IFIFO" => Self::NamedPipe,
            "S_IFSOCK" => Self::Socket,

            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileInfo {
    pub st_size: u64,
    pub st_blocks: u64,
    pub st_nlink: u64,
    pub st_mtime: DateTime<Utc>,
    pub st_birthtime: DateTime<Utc>,
    pub st_ifmt: FileType,
}

impl FileInfo {
    pub(crate) fn new(info: &mut HashMap<String, String>) -> Self {
        let mut parse = |key: &str| {
            info.remove(key)
                .unwrap_or_else(|| {
                    eprintln!("Missing key while getting the file info: {key}");
                    "".into()
                })
                .parse()
                .unwrap_or_else(|e| {
                    eprintln!("Invalid conversion to u64 while getting the file info: {e}");
                    0
                })
        };

        let to_datetime = |nanos: u64| -> DateTime<Utc> {
            let secs = nanos / 1_000_000_000;
            let dur = Duration::from_secs(secs);
            DateTime::<Utc>::from(UNIX_EPOCH + dur)
        };

        Self {
            st_size: parse("st_size"),
            st_blocks: parse("st_blocks"),
            st_nlink: parse("st_nlink"),
            st_birthtime: to_datetime(parse("st_birthtime")),
            st_mtime: to_datetime(parse("st_mtime")),
            st_ifmt: FileType::from(info.remove("st_ifmt").unwrap_or_default()),
        }
    }
}
