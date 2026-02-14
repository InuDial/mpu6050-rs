use crate::Mpu6050;
use crate::register::*;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS, T> Mpu6050<SPI, CS, T>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    T: crate::numeric::NumericType,
{
    /// 启用FIFO
    pub async fn enable_fifo(&mut self) -> Result<(), SPI::Error> {
        self.write_register(USER_CTRL, 0x40).await?; // 启用FIFO
        self.write_register(FIFO_EN, 0x78).await?; // 启用加速度计和陀螺仪数据
        Ok(())
    }

    /// 禁用FIFO
    pub async fn disable_fifo(&mut self) -> Result<(), SPI::Error> {
        self.write_register(FIFO_EN, 0x00).await?;
        self.write_register(USER_CTRL, 0x00).await?;
        Ok(())
    }

    /// 读取FIFO计数
    pub async fn read_fifo_count(&mut self) -> Result<u16, SPI::Error> {
        let mut buf = [FIFO_COUNTH | 0x80, 0, 0];
        self.cs.set_low().ok();
        self.spi.transfer_in_place(&mut buf).await?;
        self.cs.set_high().ok();
        let count = ((buf[1] as u16) << 8) | (buf[2] as u16);
        Ok(count)
    }

    /// 读取FIFO数据（固定大小缓冲区）
    pub async fn read_fifo_data(&mut self, data: &mut [u8]) -> Result<(), SPI::Error> {
        let mut buf = [0u8; 64]; // 固定大小缓冲区
        buf[0] = FIFO_R_W | 0x80;
        self.cs.set_low().ok();
        self.spi
            .transfer_in_place(&mut buf[..data.len() + 1])
            .await?;
        self.cs.set_high().ok();
        data.copy_from_slice(&buf[1..data.len() + 1]);
        Ok(())
    }
}
