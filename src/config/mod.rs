#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub use builder::ConfigBuilder as NewConfigBuilder;
pub use clock::*;
pub use config_struct::{
    AccelScale, ClockSource, ConfigBuilder, GyroScale, Mpu6050Config, calculate_sample_rate_divider,
};
pub use interrupt::*;
pub use scale::*;

mod builder;
mod clock;
mod config_struct;
mod interrupt;
mod scale;

// 陀螺仪类型
#[derive(Debug, Clone)]
pub enum GyroType {
    Gyro250 = 0x00,
    Gyro500 = 0x08,
    Gyro1000 = 0x10,
    Gyro2000 = 0x18,
}

// 加速度计类型
#[derive(Debug, Clone)]
pub enum AccelType {
    Acc2g = 0x00,
    Acc4g = 0x08,
    Acc8g = 0x10,
    Acc10g = 0x18,
}

// FIFO 使能类型
#[derive(Debug, Clone)]
pub enum FifoEnType {
    FifiDisable = 0x00,
    AccOut = 0x08,
    GyroXout = 0x10,
    GyroYout = 0x20,
    GyroZout = 0x40,
    TempOut = 0x80,
}

// 数字低通滤波器配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DlpfConfig {
    Disabled = 0x00,
    Bandwidth188Hz = 0x01,
    Bandwidth98Hz = 0x02,
    Bandwidth42Hz = 0x03,
    Bandwidth20Hz = 0x04,
    Bandwidth10Hz = 0x05,
    Bandwidth5Hz = 0x06,
}
