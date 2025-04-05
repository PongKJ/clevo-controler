use super::fd::Fd;

#[derive(Debug)]
pub enum RpalError {
    TooFrequent,
}
type Result<T> = std::result::Result<T, RpalError>;

pub struct RpalAccessor {
    fd: Fd,
    time_stamp: std::time::Instant,
    energy_comsumption: u64,
}

impl Default for RpalAccessor {
    fn default() -> Self {
        Self::new()
    }
}

impl RpalAccessor {
    pub fn new() -> Self {
        let fd = Fd::new("/sys/class/powercap/intel-rapl:0/energy_uj", libc::O_RDONLY);
        Self {
            energy_comsumption: fd.read(32).parse::<u64>().unwrap(),
            fd,
            time_stamp: std::time::Instant::now(),
        }
    }

    pub fn get_period_power(&mut self) -> Result<u64> {
        if self.time_stamp.elapsed().as_secs() < 1 {
            return Err(RpalError::TooFrequent);
        }
        let current_energy_comsumption = self.fd.read(32).parse::<u64>().unwrap();
        let period_power = (current_energy_comsumption - self.energy_comsumption)
            / self.time_stamp.elapsed().as_secs();
        self.energy_comsumption = current_energy_comsumption;
        dbg!(period_power);
        self.time_stamp = std::time::Instant::now();
        Ok(period_power)
    }
}
