use {
    interprocess::local_socket::{GenericFilePath, GenericNamespaced, Stream, prelude::*},
    std::io::{BufReader, prelude::*},
};
#[derive(Debug)]
pub enum StreamError {
    Io(String),
    Ohter(String),
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::Io(err) => write!(f, "IO Error: {}", err),
            StreamError::Ohter(err) => write!(f, "Unknow Error: {}", err),
        }
    }
}

impl From<std::io::Error> for StreamError {
    fn from(err: std::io::Error) -> Self {
        StreamError::Io(err.to_string())
    }
}

type Result<T> = std::result::Result<T, StreamError>;

pub struct LocalStream {
    stream: BufReader<Stream>,
}

impl LocalStream {
    pub fn new(name: String) -> Result<Self> {
        let name = if GenericNamespaced::is_supported() {
            name.to_ns_name::<GenericNamespaced>()?
        } else {
            name.to_fs_name::<GenericFilePath>()?
        };
        let stream = Stream::connect(name)?;
        let stream = BufReader::new(stream);
        Ok(LocalStream { stream })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        self.stream.get_mut().write_all(data)?;
        // Use '-' as a terminator
        self.stream.get_mut().write_all(b"-")?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<String> {
        let mut msg = vec![];
        self.stream.read_until(b'-', &mut msg)?;
        // Remove the msg delemiter
        msg.truncate(msg.len() - 1);
        Ok(String::from_utf8(msg).unwrap())
    }
}
