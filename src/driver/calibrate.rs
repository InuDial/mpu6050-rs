use crate::Mpu6050;

use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS, T> Mpu6050<SPI, CS, T>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    T: crate::numeric::NumericType,
{
    /// 校准传感器
    pub async fn calibrate_sensors(&mut self, cycle: u16) -> Result<(), SPI::Error> {
        self.calibrate_accel(cycle).await?;
        self.calibrate_gyro(cycle).await?;
        Ok(())
    }

    /// 校准加速度计
    pub async fn calibrate_accel(&mut self, cycle: u16) -> Result<(), SPI::Error> {
        let mut sum = (0i32, 0i32, 0i32);

        for _ in 0..cycle {
            let (x, y, z) = self.read_accel_raw().await?;
            sum.0 += x as i32;
            sum.1 += y as i32;
            sum.2 += z as i32;
            Timer::after_micros(500).await;
        }
        let avg = (
            (sum.0 / cycle as i32) as i16,
            (sum.1 / cycle as i32) as i16,
            ((sum.2 / cycle as i32) - 16384) as i16, // 减去重力加速度
        );
        self.accel_offset = avg;
        Ok(())
    }

    /// 校准陀螺仪
    pub async fn calibrate_gyro(&mut self, cycle: u16) -> Result<(), SPI::Error> {
        let mut sum = (0i32, 0i32, 0i32);
        for _ in 0..cycle {
            let (x, y, z) = self.read_gyro_raw().await?;
            sum.0 += x as i32;
            sum.1 += y as i32;
            sum.2 += z as i32;
            Timer::after_micros(500).await;
        }
        let avg = (
            (sum.0 / cycle as i32) as i16,
            (sum.1 / cycle as i32) as i16,
            (sum.2 / cycle as i32) as i16,
        );
        self.gyro_offset = avg;
        Ok(())
    }
}
