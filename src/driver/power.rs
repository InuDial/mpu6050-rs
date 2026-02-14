use crate::Mpu6050;
use crate::config::*;
use crate::register::*;
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS, T> Mpu6050<SPI, CS, T>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    T: crate::numeric::NumericType,
{
    /// 进入睡眠模式
    pub async fn sleep(&mut self) -> Result<(), SPI::Error> {
        let current = self.read_register(PWR_MGMT_1).await?;
        self.write_register(PWR_MGMT_1, current | PowerMode::Sleep as u8)
            .await?;
        Ok(())
    }

    /// 唤醒设备
    pub async fn wake(&mut self) -> Result<(), SPI::Error> {
        let current = self.read_register(PWR_MGMT_1).await?;
        self.write_register(PWR_MGMT_1, current & !(PowerMode::Sleep as u8))
            .await?;
        Timer::after_millis(100).await;
        Ok(())
    }

    /// 重置设备
    pub async fn reset(&mut self) -> Result<(), SPI::Error> {
        self.write_register(PWR_MGMT_1, 0x80).await?;
        Timer::after_millis(100).await;
        Ok(())
    }
}
