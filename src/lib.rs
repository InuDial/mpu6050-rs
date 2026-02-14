#![no_std]
#![no_main]

pub mod config;
pub mod driver;
pub mod error;
pub mod numeric;
pub mod register;
pub mod util;

pub use crate::config::Mpu6050Config;
pub use crate::error::{DeviceStatus, Mpu6050Error, Result, SensorStatus, SensorType};
pub use crate::numeric::{FixedI8F24, FixedI16F16, NumericConverter, NumericType};

pub const DEG2RAD: f32 = core::f32::consts::PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / core::f32::consts::PI;

/// MPU6050 数据快照结构体（泛型版本）
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SensorData<T: NumericType> {
    pub accel: (T, T, T),
    pub gyro: (T, T, T),
    pub temp: T,
}

impl<T: NumericType> Default for SensorData<T> {
    fn default() -> Self {
        Self {
            accel: (T::zero(), T::zero(), T::zero()),
            gyro: (T::zero(), T::zero(), T::zero()),
            temp: T::zero(),
        }
    }
}

/// 向后兼容的f32版本
pub type SensorDataF32 = SensorData<f32>;

/// 向后兼容的f32版本MPU6050
pub type Mpu6050F32<SPI, CS> = Mpu6050<SPI, CS, f32>;

/// 向后兼容的f32版本Builder
pub type Mpu6050BuilderF32<SPI, CS> = Mpu6050Builder<SPI, CS, f32>;

/// MPU6050 链式构建器
pub struct Mpu6050Builder<SPI, CS, T: NumericType = f32> {
    spi: Option<SPI>,
    cs: Option<CS>,
    config: Mpu6050Config,
    accel_offset: (i16, i16, i16),
    gyro_offset: (i16, i16, i16),
    initial_attitude: (T, T, T),
}

impl<SPI, CS, T: NumericType> Default for Mpu6050Builder<SPI, CS, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SPI, CS, T: NumericType> Mpu6050Builder<SPI, CS, T> {
    pub fn new() -> Self {
        Self {
            spi: None,
            cs: None,
            config: Mpu6050Config::default(),
            accel_offset: (0, 0, 0),
            gyro_offset: (0, 0, 0),
            initial_attitude: (T::zero(), T::zero(), T::zero()),
        }
    }
    pub fn spi(mut self, spi: SPI) -> Self {
        self.spi = Some(spi);
        self
    }
    pub fn cs(mut self, cs: CS) -> Self {
        self.cs = Some(cs);
        self
    }
    pub fn config(mut self, config: Mpu6050Config) -> Self {
        self.config = config;
        self
    }
    pub fn accel_offset(mut self, offset: (i16, i16, i16)) -> Self {
        self.accel_offset = offset;
        self
    }
    pub fn gyro_offset(mut self, offset: (i16, i16, i16)) -> Self {
        self.gyro_offset = offset;
        self
    }
    pub fn initial_attitude(mut self, pitch: T, roll: T, yaw: T) -> Self {
        self.initial_attitude = (pitch, roll, yaw);
        self
    }
    pub fn build(self) -> Mpu6050<SPI, CS, T> {
        Mpu6050 {
            spi: self.spi.expect("SPI未设置"),
            cs: self.cs.expect("CS未设置"),
            config: self.config,
            accel_offset: self.accel_offset,
            gyro_offset: self.gyro_offset,
            // last_update: None,
            pitch: self.initial_attitude.0,
            roll: self.initial_attitude.1,
            yaw: self.initial_attitude.2,
        }
    }
}

/// MPU6050 主结构体
pub struct Mpu6050<SPI, CS, T: NumericType = f32> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
    pub(crate) config: Mpu6050Config,
    pub(crate) accel_offset: (i16, i16, i16),
    pub(crate) gyro_offset: (i16, i16, i16),
    // pub(crate) last_update: Option<u64>,
    pub pitch: T,
    pub roll: T,
    pub yaw: T,
}
