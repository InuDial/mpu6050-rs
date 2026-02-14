use crate::register::*;
use crate::{Mpu6050, SensorData, numeric::NumericType};

use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS, T> Mpu6050<SPI, CS, T>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
    T: NumericType,
{
    // ================== 初始化与校准 ==================
    /// 读取设备ID（WHO_AM_I寄存器），用于检测设备是否连接正常。
    pub async fn who_am_i(&mut self) -> Result<u8, SPI::Error> {
        let mut buf = [WHO_AM_I | 0x80, 0];
        self.cs.set_low().ok();
        self.spi.transfer_in_place(&mut buf).await?;
        self.cs.set_high().ok();
        Ok(buf[1])
    }

    // ================== 数据采集 ==================
    /// 读取原始加速度计数据（三轴，单位：原始ADC）
    pub async fn read_accel_raw(&mut self) -> Result<(i16, i16, i16), SPI::Error> {
        let mut buf = [ACCEL_XOUT_H | 0x80, 0, 0, 0, 0, 0, 0];
        self.cs.set_low().ok();
        self.spi.transfer_in_place(&mut buf).await?;
        self.cs.set_high().ok();
        let x = ((buf[1] as i16) << 8) | (buf[2] as i16);
        let y = ((buf[3] as i16) << 8) | (buf[4] as i16);
        let z = ((buf[5] as i16) << 8) | (buf[6] as i16);
        Ok((x, y, z))
    }

    /// 读取校准后的加速度计数据（三轴，单位：m/s²）
    pub async fn read_accel(&mut self) -> Result<(T, T, T), SPI::Error> {
        let (x, y, z) = self.read_accel_raw().await?;
        let x = x - self.accel_offset.0;
        let y = y - self.accel_offset.1;
        let z = z - self.accel_offset.2;
        let scale = self.config.accel_scale.get_scale_factor();
        let x = T::from_f32((x as f32) * 9.81 / scale);
        let y = T::from_f32((y as f32) * 9.81 / scale);
        let z = T::from_f32((z as f32) * 9.81 / scale);
        Ok((x, y, z))
    }

    /// 读取原始陀螺仪数据（三轴，单位：原始ADC）
    pub async fn read_gyro_raw(&mut self) -> Result<(i16, i16, i16), SPI::Error> {
        self.cs.set_low().ok();
        let mut buf = [GYRO_XOUT_H | 0x80, 0, 0, 0, 0, 0, 0];
        self.spi.transfer_in_place(&mut buf).await?;
        self.cs.set_high().ok();
        let x = ((buf[1] as i16) << 8) | (buf[2] as i16);
        let y = ((buf[3] as i16) << 8) | (buf[4] as i16);
        let z = ((buf[5] as i16) << 8) | (buf[6] as i16);
        Ok((x, y, z))
    }

    /// 读取校准后的陀螺仪数据（三轴，单位：rad/s）
    pub async fn read_gyro(&mut self) -> Result<(T, T, T), SPI::Error> {
        let (x, y, z) = self.read_gyro_raw().await?;
        let x = x - self.gyro_offset.0;
        let y = y - self.gyro_offset.1;
        let z = z - self.gyro_offset.2;
        let scale = self.config.gyro_scale.get_scale_factor();
        let x = T::from_f32((x as f32) * core::f32::consts::PI / (180.0 * scale));
        let y = T::from_f32((y as f32) * core::f32::consts::PI / (180.0 * scale));
        let z = T::from_f32((z as f32) * core::f32::consts::PI / (180.0 * scale));
        Ok((x, y, z))
    }

    /// 读取温度（单位：摄氏度）
    pub async fn read_temp(&mut self) -> Result<T, SPI::Error> {
        let mut buf = [TEMP_OUT_H | 0x80, 0, 0];
        self.cs.set_low().ok();
        self.spi.transfer_in_place(&mut buf).await?;
        self.cs.set_high().ok();
        let raw = ((buf[1] as i16) << 8) | (buf[2] as i16);
        Ok(T::from_f32((raw as f32) / TEMP_SCALE + TEMP_OFFSET))
    }

    /// 读取所有传感器数据（加速度、陀螺仪、温度，单位：物理量）
    pub async fn read_all(&mut self) -> Result<SensorData<T>, SPI::Error> {
        let accel = self.read_accel().await?;
        let gyro = self.read_gyro().await?;
        let temp = self.read_temp().await?;

        Ok(SensorData { accel, gyro, temp })
    }

    // ================== 姿态解算与滤波 ==================
    /// 仅用加速度计计算 pitch/roll（静态欧拉角，弧度）
    pub async fn calculate_pitch_roll_from_accel(&mut self) -> Result<(T, T), SPI::Error> {
        let (ax, ay, az) = self.read_accel().await?;
        let pitch_a = T::atan2(ax, (ay * ay + az * az).sqrt());
        let roll_a = T::atan2(ay, az);
        Ok((pitch_a, roll_a))
    }

    /// 仅用陀螺仪积分更新 pitch/roll/yaw（弧度）
    pub async fn integrate_gyro(&mut self, dt: T) -> Result<(T, T, T), SPI::Error> {
        let (gx, gy, gz) = self.read_gyro().await?;
        self.pitch += gx * dt;
        self.roll += gy * dt;
        self.yaw += gz * dt;
        Ok((self.pitch, self.roll, self.yaw))
    }

    /// 互补滤波融合加速度计和陀螺仪（弧度）
    pub fn complementary_filter(
        &mut self,
        acc_pitch: T,
        acc_roll: T,
        pitch_g: T,
        roll_g: T,
        yaw_g: T,
        alpha: T,
    ) {
        let one = T::one();
        self.pitch = alpha * pitch_g + (one - alpha) * acc_pitch;
        self.roll = alpha * roll_g + (one - alpha) * acc_roll;
        self.yaw = yaw_g;
    }

    /// 获取当前欧拉角（单位：度）
    pub fn get_euler_angles(&self) -> (T, T, T) {
        (
            self.pitch * T::rad_to_deg(),
            self.roll * T::rad_to_deg(),
            self.yaw * T::rad_to_deg(),
        )
    }

    /// 更新姿态角（pitch/roll/yaw），融合加速度计和陀螺仪数据
    ///
    /// # 参数
    /// - `dt`: 两次采样之间的时间间隔（单位：秒）
    /// - `alpha`: 互补滤波系数，范围 0~1，越大越依赖陀螺仪，越小越依赖加速度计
    ///
    /// # 返回
    /// - `Result<(), SPI::Error>`: 操作结果，可能包含SPI通信错误
    ///
    /// # 算法说明
    /// 1. 读取加速度计和陀螺仪原始数据
    /// 2. 用加速度计计算 pitch/roll 的静态角度
    /// 3. 用互补滤波融合陀螺仪积分和加速度计角度，更新 pitch/roll
    /// 4. yaw 仅用陀螺仪积分
    pub async fn update(&mut self, dt: T, alpha: T) -> Result<(), SPI::Error> {
        // 陀螺仪计算欧拉角
        let (pitch_g, roll_g, yaw_g) = self.integrate_gyro(dt).await?;
        // 加速度计计算欧拉角
        let (pitch_a, roll_a) = self.calculate_pitch_roll_from_accel().await?;

        self.complementary_filter(pitch_a, roll_a, pitch_g, roll_g, yaw_g, alpha);

        Ok(())
    }
}
