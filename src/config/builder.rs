//! 配置构建器
//! 
//! 提供类型安全的配置构建功能

use super::*;
use crate::error::{ConfigValidation, Mpu6050Error};

/// 改进的配置构建器
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    accel_scale: AccelScale,
    gyro_scale: GyroScale,
    dlpf_config: DlpfConfig,
    sample_rate: u16,
    clock_source: ClockSource,
    enable_interrupts: bool,
    enable_fifo: bool,
    low_power_mode: bool,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            accel_scale: AccelScale::Scale2G,
            gyro_scale: GyroScale::Scale250,
            dlpf_config: DlpfConfig::Bandwidth42Hz,
            sample_rate: 1000,
            clock_source: ClockSource::Internal,
            enable_interrupts: false,
            enable_fifo: false,
            low_power_mode: false,
        }
    }

    /// 设置加速度计量程
    pub fn accel_scale(mut self, scale: AccelScale) -> Self {
        self.accel_scale = scale;
        self
    }

    /// 设置陀螺仪量程
    pub fn gyro_scale(mut self, scale: GyroScale) -> Self {
        self.gyro_scale = scale;
        self
    }

    /// 设置数字低通滤波器配置
    pub fn dlpf_config(mut self, config: DlpfConfig) -> Self {
        self.dlpf_config = config;
        self
    }

    /// 设置采样率（Hz）
    /// 
    /// # 参数
    /// - `rate`: 采样率，范围 4-1000 Hz
    /// 
    /// # 注意
    /// 实际采样率受DLPF配置影响
    pub fn sample_rate(mut self, rate: u16) -> Self {
        self.sample_rate = rate.clamp(4, 1000);
        self
    }

    /// 设置时钟源
    pub fn clock_source(mut self, source: ClockSource) -> Self {
        self.clock_source = source;
        self
    }

    /// 启用中断
    pub fn enable_interrupts(mut self, enable: bool) -> Self {
        self.enable_interrupts = enable;
        self
    }

    /// 启用FIFO
    pub fn enable_fifo(mut self, enable: bool) -> Self {
        self.enable_fifo = enable;
        self
    }

    /// 启用低功耗模式
    pub fn low_power_mode(mut self, enable: bool) -> Self {
        self.low_power_mode = enable;
        self
    }

    /// 构建配置
    pub fn build(self) -> core::result::Result<Mpu6050Config, Mpu6050Error<()>> {
        let config = Mpu6050Config {
            accel_scale: self.accel_scale,
            gyro_scale: self.gyro_scale,
            dlpf_config: self.dlpf_config,
            sample_rate: self.sample_rate,
            clock_source: self.clock_source,
            enable_interrupts: self.enable_interrupts,
            enable_fifo: self.enable_fifo,
            low_power_mode: self.low_power_mode,
        };

        config.validate()?;
        Ok(config)
    }

    /// 构建配置（不验证）
    pub fn build_unchecked(self) -> Mpu6050Config {
        Mpu6050Config {
            accel_scale: self.accel_scale,
            gyro_scale: self.gyro_scale,
            dlpf_config: self.dlpf_config,
            sample_rate: self.sample_rate,
            clock_source: self.clock_source,
            enable_interrupts: self.enable_interrupts,
            enable_fifo: self.enable_fifo,
            low_power_mode: self.low_power_mode,
        }
    }
}

/// 预设配置
impl ConfigBuilder {
    /// 高精度配置（低噪声，高分辨率）
    pub fn high_precision() -> Self {
        Self::new()
            .accel_scale(AccelScale::Scale2G)
            .gyro_scale(GyroScale::Scale250)
            .dlpf_config(DlpfConfig::Bandwidth5Hz)
            .sample_rate(100)
    }

    /// 高速配置（高采样率）
    pub fn high_speed() -> Self {
        Self::new()
            .accel_scale(AccelScale::Scale4G)
            .gyro_scale(GyroScale::Scale500)
            .dlpf_config(DlpfConfig::Bandwidth42Hz)
            .sample_rate(1000)
    }

    /// 低功耗配置
    pub fn low_power() -> Self {
        Self::new()
            .accel_scale(AccelScale::Scale2G)
            .gyro_scale(GyroScale::Scale250)
            .dlpf_config(DlpfConfig::Bandwidth20Hz)
            .sample_rate(50)
            .low_power_mode(true)
    }

    /// 运动检测配置
    pub fn motion_detection() -> Self {
        Self::new()
            .accel_scale(AccelScale::Scale8G)
            .gyro_scale(GyroScale::Scale1000)
            .dlpf_config(DlpfConfig::Bandwidth42Hz)
            .sample_rate(200)
            .enable_interrupts(true)
    }

    /// 姿态估计配置
    pub fn attitude_estimation() -> Self {
        Self::new()
            .accel_scale(AccelScale::Scale4G)
            .gyro_scale(GyroScale::Scale500)
            .dlpf_config(DlpfConfig::Bandwidth20Hz)
            .sample_rate(200)
    }
}

impl ConfigValidation for Mpu6050Config {
    fn validate(&self) -> core::result::Result<(), Mpu6050Error<()>> {
        // 验证采样率范围
        if self.sample_rate < 4 || self.sample_rate > 1000 {
            return Err(Mpu6050Error::InvalidConfig);
        }

        // 验证低功耗模式下的配置
        if self.low_power_mode && self.sample_rate > 100 {
            return Err(Mpu6050Error::InvalidConfig);
        }

        // 验证FIFO和中断的兼容性
        if self.enable_fifo && !self.enable_interrupts {
            // FIFO通常需要中断来指示数据就绪
            // 这里只是警告，不是错误
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_default() {
        let config = ConfigBuilder::new().build_unchecked();
        assert_eq!(config.accel_scale, AccelScale::Scale2G);
        assert_eq!(config.gyro_scale, GyroScale::Scale250);
        assert_eq!(config.sample_rate, 1000);
    }

    #[test]
    fn test_config_validation() {
        // 有效配置
        let config = ConfigBuilder::new().sample_rate(100).build();
        assert!(config.is_ok());

        // 无效采样率
        let config = ConfigBuilder::new().sample_rate(2000).build();
        assert!(config.is_err());
    }

    #[test]
    fn test_preset_configs() {
        let high_precision = ConfigBuilder::high_precision().build_unchecked();
        assert_eq!(high_precision.accel_scale, AccelScale::Scale2G);
        assert_eq!(high_precision.sample_rate, 100);

        let high_speed = ConfigBuilder::high_speed().build_unchecked();
        assert_eq!(high_speed.sample_rate, 1000);

        let low_power = ConfigBuilder::low_power().build_unchecked();
        assert!(low_power.low_power_mode);
    }
}
