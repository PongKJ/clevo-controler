use {
    clevo_fancontrol::communicate::proto::{Reply, Request},
    interprocess::local_socket::{
        GenericFilePath, GenericNamespaced, ListenerOptions, Stream, prelude::*,
    },
    std::io::{self, BufReader, prelude::*},
};

use crate::communicate::handler::*;

// TODO: dispatch to different handlers based on the index
pub fn handler(request: Request) -> Reply {
    match request {
        Request::GetDesc(index) => handle_get_desc(index),
        Request::GetFreq(index) => handle_get_freq(index),
        Request::GetFanSpeed(index) => handle_get_fan_speed(index),
        _ => Reply::NotSupport,
    }
}
