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
    /// 启用中断
    pub async fn enable_interrupts(&mut self) -> Result<(), SPI::Error> {
        self.write_register(INT_ENABLE, InterruptType::DataReady as u8)
            .await?;
        Ok(())
    }

    /// 禁用中断
    pub async fn disable_interrupts(&mut self) -> Result<(), SPI::Error> {
        self.write_register(INT_ENABLE, 0x00).await?;
        Ok(())
    }

    /// 读取中断状态
    pub async fn read_interrupt_status(&mut self) -> Result<u8, SPI::Error> {
        self.read_register(INT_STATUS).await
    }
}
