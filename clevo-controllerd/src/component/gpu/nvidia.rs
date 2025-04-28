use crate::lowlevel::middleware;

#[derive(Debug)]
pub enum GpuError {
    NvmlError,
    Other(String),
}
impl From<nvml_wrapper::error::NvmlError> for GpuError {
    fn from(_err: nvml_wrapper::error::NvmlError) -> Self {
        GpuError::NvmlError
    }
}

type Result<T> = std::result::Result<T, GpuError>;

pub struct NvidiaGpu {
    nvml: nvml_wrapper::Nvml,
    pub desc: String,
    pub freq: u64,
    pub temp: u64,
    pub power: u64,
    pub fan_speed: u64,
}

impl NvidiaGpu {
    pub fn init() -> Result<Self> {
        let nvml = nvml_wrapper::Nvml::init()?;
        let device = nvml.device_by_index(0)?;
        let gpu = NvidiaGpu {
            desc: device.name()?,
            freq: device.clock(
                nvml_wrapper::enum_wrappers::device::Clock::Graphics,
                nvml_wrapper::enum_wrappers::device::ClockId::Current,
            )? as u64,
            temp: device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?
                as u64,
            power: device.power_usage()? as u64,
            fan_speed: 0,
            nvml,
        };

        Ok(gpu)
    }

    pub fn refresh(&mut self) -> Result<()> {
        let device = self.nvml.device_by_index(0)?;
        // refresh freq
        self.freq = device.clock(
            nvml_wrapper::enum_wrappers::device::Clock::Graphics,
            nvml_wrapper::enum_wrappers::device::ClockId::Current,
        )? as u64;
        // refresh temp
        self.temp =
            device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)? as u64;
        // refresh power
        self.power = device.power_usage()? as u64;
        // refresh fan speed
        let fan = middleware::fan::Fan::get_instance();
        self.fan_speed = fan.get_fan_rpm(middleware::fan::FanIndex::GPU) as u64;
        Ok(())
    }
}
