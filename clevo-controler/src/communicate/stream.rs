use {
    interprocess::local_socket::{
        GenericFilePath, GenericNamespaced, ListenerOptions, Stream, prelude::*,
    },
    std::io::{BufReader, prelude::*},
};
#[derive(Debug)]
pub enum StreamError {
    Io(String),
    Other(String),
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::Io(err) => write!(f, "IO Error: {}", err),
            StreamError::Other(err) => write!(f, "Unknow Error: {}", err),
        }
    }
}

impl From<std::io::Error> for StreamError {
    fn from(err: std::io::Error) -> Self {
        StreamError::Io(err.to_string())
    }
}

type Result<T> = std::result::Result<T, StreamError>;

pub struct SocketStream(Stream);

impl SocketStream {
    pub fn new(name: &str) -> Result<Self> {
        let name = if GenericNamespaced::is_supported() {
            name.to_ns_name::<GenericNamespaced>()?
        } else {
            name.to_fs_name::<GenericFilePath>()?
        };
        let stream = Stream::connect(name)?;
        Ok(stream)
    }

    pub fn write(&mut self, buffer: &str) -> Result<()> {
        self.0.write_vectored(buffer);
        Ok(())
    }

    pub fn read(&mut self, length: usize) -> Result<String> {
        let mut msg = vec![0; length]; // Pre-allocate a buffer of the specified length
        self.0.read_exact(&mut msg)?;
        Ok(String::from_utf8(msg).unwrap())
    }
}

pub struct StreamListener(interprocess::local_socket::Listener);

impl StreamListener {
    pub fn new(socket_name: &str) -> Result<Self> {
        let socket_name = if GenericNamespaced::is_supported() {
            socket_name.to_ns_name::<GenericNamespaced>()?
        } else {
            socket_name.to_fs_name::<GenericFilePath>()?
        };
        let opts = ListenerOptions::new().name(socket_name);
        let listener = opts.create_sync()?;
        Ok(listener)
    }

    pub fn accept(&mut self) -> Result<SocketStream> {
        match self.listener.accept() {
            Ok(stream) => {
                let stream_client = SocketStream(stream);
                Ok(stream_client)
            }
            Err(err) => Err(StreamError::Io(err.to_string())),
        }
    }
}
