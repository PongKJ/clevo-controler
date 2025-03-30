use {
    super::proto::Msg,
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

pub struct StreamClient {
    stream: BufReader<Stream>,
}

impl StreamClient {
    pub fn new(name: &str) -> Result<Self> {
        let name = if GenericNamespaced::is_supported() {
            name.to_ns_name::<GenericNamespaced>()?
        } else {
            name.to_fs_name::<GenericFilePath>()?
        };
        let stream = Stream::connect(name)?;
        let stream = BufReader::new(stream);
        Ok(StreamClient { stream })
    }

    pub fn write(&mut self, msg: impl Msg) -> Result<()> {
        self.stream
            .get_mut()
            .write_all(msg.get_raw_string().as_bytes())?;
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

pub struct StreamListener {
    listener: interprocess::local_socket::Listener,
}

impl StreamListener {
    pub fn new(socket_name: &str) -> Result<Self> {
        let socket_name = if GenericNamespaced::is_supported() {
            socket_name.to_ns_name::<GenericNamespaced>()?
        } else {
            socket_name.to_fs_name::<GenericFilePath>()?
        };
        let opts = ListenerOptions::new().name(socket_name);
        let listener = opts.create_sync()?;
        Ok(StreamListener { listener })
    }

    pub fn accept(&mut self) -> Result<StreamClient> {
        match self.listener.accept() {
            Ok(stream) => {
                let stream_client = StreamClient {
                    stream: BufReader::new(stream),
                };
                Ok(stream_client)
            }
            Err(err) => Err(StreamError::Io(err.to_string())),
        }
    }
}
