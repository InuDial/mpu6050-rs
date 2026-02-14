//! 错误类型定义
//! 
//! 提供统一的错误处理机制

use core::fmt;

/// MPU6050库的统一错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum Mpu6050Error<SpiError> {
    /// SPI通信错误
    Spi(SpiError),
    /// 设备未找到或ID不匹配
    DeviceNotFound,
    /// 配置无效
    InvalidConfig,
    /// 校准失败
    CalibrationFailed,
    /// 数据读取失败
    DataReadFailed,
    /// FIFO错误
    FifoError,
    /// 中断配置错误
    InterruptError,
    /// 数值转换错误
    ConversionError,
}

impl<SpiError> fmt::Display for Mpu6050Error<SpiError>
where
    SpiError: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mpu6050Error::Spi(e) => write!(f, "SPI error: {e}"),
            Mpu6050Error::DeviceNotFound => write!(f, "MPU6050 device not found"),
            Mpu6050Error::InvalidConfig => write!(f, "Invalid configuration"),
            Mpu6050Error::CalibrationFailed => write!(f, "Sensor calibration failed"),
            Mpu6050Error::DataReadFailed => write!(f, "Failed to read sensor data"),
            Mpu6050Error::FifoError => write!(f, "FIFO operation error"),
            Mpu6050Error::InterruptError => write!(f, "Interrupt configuration error"),
            Mpu6050Error::ConversionError => write!(f, "Numeric conversion error"),
        }
    }
}

impl<SpiError> From<SpiError> for Mpu6050Error<SpiError> {
    fn from(error: SpiError) -> Self {
        Mpu6050Error::Spi(error)
    }
}

/// 结果类型别名
pub type Result<T, SpiError> = core::result::Result<T, Mpu6050Error<SpiError>>;

/// 配置验证trait
pub trait ConfigValidation {
    /// 验证配置是否有效
    fn validate(&self) -> core::result::Result<(), Mpu6050Error<()>>;
}

/// 设备状态枚举
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum DeviceStatus {
    /// 未初始化
    #[default]
    Uninitialized,
    /// 已初始化
    Initialized,
    /// 已校准
    Calibrated,
    /// 错误状态
    Error,
}


/// 传感器类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    /// 加速度计
    Accelerometer,
    /// 陀螺仪
    Gyroscope,
    /// 温度传感器
    Temperature,
}

/// 传感器状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorStatus {
    /// 传感器类型
    pub sensor_type: SensorType,
    /// 是否已校准
    pub calibrated: bool,
    /// 是否启用
    pub enabled: bool,
    /// 最后更新时间戳（可选）
    pub last_update: Option<u64>,
}

impl SensorStatus {
    /// 创建新的传感器状态
    pub fn new(sensor_type: SensorType) -> Self {
        Self {
            sensor_type,
            calibrated: false,
            enabled: true,
            last_update: None,
        }
    }
    
    /// 标记为已校准
    pub fn mark_calibrated(&mut self) {
        self.calibrated = true;
    }
    
    /// 更新时间戳
    pub fn update_timestamp(&mut self, timestamp: u64) {
        self.last_update = Some(timestamp);
    }
}
