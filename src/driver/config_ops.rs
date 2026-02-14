use crate::Mpu6050;
use crate::config::*;
use crate::register::*;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS, T> Mpu6050<SPI, CS, T>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    T: crate::numeric::NumericType,
{
    /// 设置加速度计量程
    pub async fn set_accel_scale(&mut self, scale: AccelScale) -> Result<(), SPI::Error> {
        self.config.accel_scale = scale;
        self.write_register(ACCEL_CONFIG, scale as u8).await?;
        Ok(())
    }

    /// 设置陀螺仪量程
    pub async fn set_gyro_scale(&mut self, scale: GyroScale) -> Result<(), SPI::Error> {
        self.config.gyro_scale = scale;
        self.write_register(GYRO_CONFIG, scale as u8).await?;
        Ok(())
    }

    /// 设置时钟源
    pub async fn set_clock_source(&mut self, clock_source: ClockSource) -> Result<(), SPI::Error> {
        self.config.clock_source = clock_source;
        let current = self.read_register(PWR_MGMT_1).await?;
        let new_value = (current & 0xF8) | (clock_source as u8);
        self.write_register(PWR_MGMT_1, new_value).await?;
        Ok(())
    }
}
