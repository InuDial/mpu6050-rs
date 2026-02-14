use crate::Mpu6050;

use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS, T> Mpu6050<SPI, CS, T>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    T: crate::numeric::NumericType,
{
    /// 读取寄存器
    pub async fn read_register(&mut self, reg: u8) -> Result<u8, SPI::Error> {
        let mut buf = [reg | 0x80, 0];
        self.cs.set_low().ok();
        self.spi.transfer_in_place(&mut buf).await?;
        self.cs.set_high().ok();
        Ok(buf[1])
    }

    /// 写入寄存器
    pub async fn write_register(&mut self, reg: u8, val: u8) -> Result<(), SPI::Error> {
        let buf = [reg & 0x7F, val];
        self.cs.set_low().ok();
        self.spi.write(&buf).await?;
        self.cs.set_high().ok();
        Ok(())
    }
}
