pub use super::clock::{ClockSource, PowerMode};
pub use super::interrupt::InterruptType;
pub use super::scale::{AccelScale, GyroScale};

/// ConfigBuilder 支持链式调用的构建器模式
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    accel_scale: AccelScale,
    gyro_scale: GyroScale,
    dlpf_config: super::DlpfConfig,
    sample_rate: u16,
    clock_source: ClockSource,
    enable_interrupts: bool,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            accel_scale: AccelScale::Scale2G,
            gyro_scale: GyroScale::Scale250,
            dlpf_config: super::DlpfConfig::Bandwidth42Hz,
            sample_rate: 1000,
            clock_source: ClockSource::Internal,
            enable_interrupts: false,
        }
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn accel_scale(mut self, accel_scale: AccelScale) -> Self {
        self.accel_scale = accel_scale;
        self
    }
    pub fn gyro_scale(mut self, gyro_scale: GyroScale) -> Self {
        self.gyro_scale = gyro_scale;
        self
    }
    pub fn dlpf_config(mut self, dlpf_config: super::DlpfConfig) -> Self {
        self.dlpf_config = dlpf_config;
        self
    }
    pub fn sample_rate(mut self, sample_rate: u16) -> Self {
        self.sample_rate = sample_rate;
        self
    }
    pub fn clock_source(mut self, clock_source: ClockSource) -> Self {
        self.clock_source = clock_source;
        self
    }
    pub fn enable_interrupts(mut self, enable_interrupts: bool) -> Self {
        self.enable_interrupts = enable_interrupts;
        self
    }
    pub fn build(self) -> Mpu6050Config {
        Mpu6050Config {
            accel_scale: self.accel_scale,
            gyro_scale: self.gyro_scale,
            dlpf_config: self.dlpf_config,
            sample_rate: self.sample_rate,
            clock_source: self.clock_source,
            enable_interrupts: self.enable_interrupts,
            enable_fifo: false,
            low_power_mode: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mpu6050Config {
    pub accel_scale: AccelScale,
    pub gyro_scale: GyroScale,
    pub dlpf_config: super::DlpfConfig,
    pub sample_rate: u16,
    pub clock_source: ClockSource,
    pub enable_interrupts: bool,
    pub enable_fifo: bool,
    pub low_power_mode: bool,
}

impl Default for Mpu6050Config {
    fn default() -> Self {
        Self {
            accel_scale: AccelScale::Scale2G,
            gyro_scale: GyroScale::Scale250,
            dlpf_config: super::DlpfConfig::Bandwidth42Hz,
            sample_rate: 1000,
            clock_source: ClockSource::Internal,
            enable_interrupts: false,
            enable_fifo: false,
            low_power_mode: false,
        }
    }
}

impl Mpu6050Config {
    pub fn new(builder: ConfigBuilder) -> Self {
        builder.build()
    }
}

pub fn calculate_sample_rate_divider(desired_rate: u16) -> u8 {
    // MPU6050内部时钟为1kHz，采样率 = 1000 / (1 + SMPLRT_DIV)
    if desired_rate >= 1000 {
        0
    } else if desired_rate <= 1 {
        255
    } else {
        ((1000 / desired_rate) - 1) as u8
    }
}
