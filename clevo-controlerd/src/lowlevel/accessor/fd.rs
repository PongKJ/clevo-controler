use std::ffi::CString;

use libc::c_int;

const BYTES_PER_READ: usize = 50;

#[derive(Debug)]
pub struct Fd {
    fd: i32,
}

impl Fd {
    pub fn new(path: &str, mode: c_int) -> Self {
        let fd = unsafe { libc::open(CString::new(path).unwrap().as_ptr(), mode) };
        if fd < 0 {
            panic!("Failed to open file descriptor");
        }
        Self { fd }
    }

    // At least read min_len bytes, may read less if a '\n' is been read
    pub fn read(&self, min_len: usize) -> String {
        unsafe { libc::lseek(self.fd, 0, libc::SEEK_SET) };
        let mut buffer = [0u8; BYTES_PER_READ];
        let mut read_times = 0;
        let mut data = Vec::new();
        while read_times * BYTES_PER_READ < min_len {
            let ret = unsafe {
                libc::read(
                    self.fd,
                    &mut buffer as *mut _ as *mut libc::c_void,
                    BYTES_PER_READ,
                )
            };
            if ret < 0 {
                panic!("Failed to read from file descriptor");
            } else if ret < BYTES_PER_READ as isize {
                // EOF
                data.extend_from_slice(&buffer[..ret as usize]);
                break;
            }
            read_times += 1;
            data.extend_from_slice(&buffer[..]);
        }
        let str = String::from_utf8_lossy(data.as_slice());
        str.trim().to_string()
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        todo!()
    }
}
