use {
    interprocess::local_socket::{
        GenericFilePath, GenericNamespaced, ListenerOptions, Stream, prelude::*,
    },
    std::io::{self, BufReader, prelude::*},
};

enum ListenerError {
    Io(String),
    Other(String),
}
type Result<T> = std::result::Result<T, ListenerError>;

struct LocalListener {
    listener: interprocess::local_socket::Listener,
}

impl LocalListener {
    pub fn new(name: String) -> io::Result<Self> {
        let name = if GenericNamespaced::is_supported() {
            name.to_ns_name::<GenericNamespaced>()?
        } else {
            name.to_fs_name::<GenericFilePath>()?
        };
        let opts = ListenerOptions::new().name(name);
        let listener = opts.create_sync()?;
        Ok(LocalListener { listener })
    }
}
