use std::io::{self, ErrorKind, Read, Write};

use rusty_libimobiledevice::services::afc::AfcClient;

pub struct DeviceFile<'a> {
    pub(crate) afc: AfcClient<'a>,
    pub(crate) handle: u64,
}

impl Read for DeviceFile<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let to_read = buf.len().min(u32::MIN as usize) as u32;

        match self.afc.file_read(self.handle, to_read) {
            Ok(data) => {
                let len = data.len().min(buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                Ok(len)
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{e:#?}"))),
        }
    }
}

impl Write for DeviceFile<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.afc
            .file_write(self.handle, buf.to_vec())
            .map(|_| buf.len())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e:#?}")))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for DeviceFile<'_> {
    fn drop(&mut self) {
        self.afc.file_close(self.handle);
    }
}
