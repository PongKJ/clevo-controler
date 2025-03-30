use std::{thread, time::Duration};
use x86::io;
#[derive(Debug)]
pub struct EcAccessor {}

const EC_SC_REG: u16 = 0x66;
const EC_DATA_REG: u16 = 0x62;
const EC_READ_CMD: u8 = 0x80;
const EC_WRITE_CMD: u8 = 0x81;
const EC_SC_IBF_INDEX: u8 = 1;
const EC_SC_OBF_INDEX: u8 = 0;

impl EcAccessor {
    pub fn new() -> Self {
        let ec_accessor = EcAccessor {};
        ec_accessor.init();
        ec_accessor
    }

    fn init(&self) {
        // Enable I/O port access
        unsafe {
            libc::ioperm(EC_SC_REG as u64, 1, 1);
            libc::ioperm(EC_DATA_REG as u64, 1, 1);
        }
    }

    fn inb(&self, addr: u16) -> u8 {
        let byte;
        unsafe {
            byte = io::inb(addr);
        }
        byte
    }

    fn outb(&self, addr: u16, byte: u8) {
        unsafe {
            io::outb(addr, byte);
        }
    }

    fn poll_ready(&self, addr: u16, bit: u8, value: bool) {
        let mut max_tries = 1000;
        println!("start poll");
        while max_tries > 0 {
            let status = self.inb(addr);
            dbg!(status);
            if ((status >> bit) & 1) == value as u8 {
                return;
            }
            max_tries -= 1;
            thread::sleep(Duration::from_millis(1));
        }
        eprintln!("EC poll ready timeout");
    }

    pub fn read_byte(&self, addr: u8) -> u8 {
        self.cmd_read(EC_READ_CMD, addr)
    }

    pub fn write_byte(&self, addr: u8, byte: u8) {
        self.cmd_write(EC_WRITE_CMD, addr, byte);
    }

    pub fn cmd_read(&self, cmd: u8, addr: u8) -> u8 {
        self.poll_ready(EC_SC_REG, EC_SC_IBF_INDEX, false);
        self.outb(EC_SC_REG, cmd);
        self.poll_ready(EC_SC_REG, EC_SC_IBF_INDEX, false);
        self.outb(EC_DATA_REG, addr);
        self.poll_ready(EC_SC_REG, EC_SC_OBF_INDEX, true);
        self.inb(EC_DATA_REG)
    }

    pub fn cmd_write(&self, cmd: u8, addr: u8, byte: u8) {
        self.poll_ready(EC_SC_REG, EC_SC_IBF_INDEX, false);
        self.outb(EC_SC_REG, cmd);
        self.poll_ready(EC_SC_REG, EC_SC_IBF_INDEX, false);
        self.outb(EC_DATA_REG, addr);
        self.poll_ready(EC_SC_REG, EC_SC_IBF_INDEX, false);
        self.outb(EC_DATA_REG, byte);
    }
}
