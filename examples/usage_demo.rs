//! MPU6050使用示例
//! 
//! 展示如何使用新的泛型API和配置系统
//! 
//! 注意：这个文件仅用于展示API使用方法，不会被编译为可执行文件

#![no_std]
#![no_main]

use mpu6050::{
    config::NewConfigBuilder,
    Mpu6050, SensorData, NumericType,
    FixedI16F16, // 定点数类型
};

// 在实际项目中，你需要替换这些为具体的HAL类型
// 例如：
// use stm32f4xx_hal::{spi::Spi, gpio::gpioa::PA4};
// type MySpi = Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>;
// type MyCs = PA4<Output<PushPull>>;

/// 使用f32浮点数的示例
async fn example_with_f32() -> Result<(), ()> {
    // 在实际项目中，这里会是真实的SPI和CS引脚
    // let spi = MySpi::new(...);
    // let cs = MyCs::new(...);
    
    // 使用新的配置构建器
    let config = NewConfigBuilder::high_precision()
        .sample_rate(200)
        .enable_interrupts(true)
        .build()
        .map_err(|_| ())?;
    
    // 创建MPU6050实例（默认使用f32）
    // let mut mpu: Mpu6050<_, _, f32> = Mpu6050::new(spi, cs, config);
    
    // 初始化设备
    // mpu.init_with_config().await.map_err(|_| ())?;
    
    // 校准传感器
    // mpu.calibrate_sensors(1000).await.map_err(|_| ())?;
    
    // 读取传感器数据
    // let data: SensorData<f32> = mpu.read_all().await.map_err(|_| ())?;
    
    // 处理数据
    // let (ax, ay, az) = data.accel;
    // let (gx, gy, gz) = data.gyro;
    // let temp = data.temp;
    
    // 在实际应用中，这里会有数据处理逻辑
    // 例如：姿态解算、滤波等
    
    Ok(())
}

/// 使用定点数的示例（适用于资源受限的系统）
async fn example_with_fixed_point() -> Result<(), ()> {
    // 低功耗配置
    let config = NewConfigBuilder::low_power()
        .sample_rate(50)
        .build()
        .map_err(|_| ())?;
    
    // 创建使用定点数的MPU6050实例
    // let mut mpu: Mpu6050<_, _, FixedI16F16> = Mpu6050::new(spi, cs, config);
    
    // 初始化和校准
    // mpu.init_with_config().await.map_err(|_| ())?;
    // mpu.calibrate_sensors(500).await.map_err(|_| ())?;
    
    // 读取数据
    // let data: SensorData<FixedI16F16> = mpu.read_all().await.map_err(|_| ())?;
    
    // 定点数运算
    // let accel_magnitude = {
    //     let (ax, ay, az) = data.accel;
    //     (ax * ax + ay * ay + az * az).sqrt()
    // };
    
    // 转换为浮点数进行显示或进一步处理
    // let magnitude_f32 = accel_magnitude.to_f32();
    
    Ok(())
}

/// 高级配置示例
async fn example_advanced_config() -> Result<(), ()> {
    // 自定义配置
    let config = NewConfigBuilder::new()
        .accel_scale(mpu6050::config::AccelScale::Scale4G)
        .gyro_scale(mpu6050::config::GyroScale::Scale500)
        .dlpf_config(mpu6050::config::DlpfConfig::Bandwidth20Hz)
        .sample_rate(100)
        .enable_fifo(true)
        .enable_interrupts(true)
        .build()
        .map_err(|_| ())?;
    
    // let mut mpu: Mpu6050<_, _, f32> = Mpu6050::new(spi, cs, config);
    
    // 初始化
    // mpu.init_with_config().await.map_err(|_| ())?;
    
    // 校准
    // mpu.calibrate_sensors(2000).await.map_err(|_| ())?;
    
    // 连续读取数据
    // for _ in 0..10 {
    //     let data = mpu.read_all().await.map_err(|_| ())?;
    //     
    //     // 处理数据...
    //     // 在实际应用中，这里可能包括：
    //     // - 数据滤波
    //     // - 姿态解算
    //     // - 运动检测
    //     // - 数据记录等
    //     
    //     // 模拟延时
    //     // embassy_time::Timer::after_millis(10).await;
    // }
    
    Ok(())
}

/// 错误处理示例
async fn example_error_handling() -> Result<(), mpu6050::Mpu6050Error<()>> {
    // 使用Result类型进行错误处理
    let config = NewConfigBuilder::new()
        .sample_rate(100) // 有效的采样率
        .build()?; // 使用?操作符处理错误
    
    // let mut mpu: Mpu6050<_, _, f32> = Mpu6050::new(spi, cs, config);
    
    // 使用统一的错误类型
    // match mpu.init_with_config().await {
    //     Ok(_) => {
    //         // 初始化成功
    //     }
    //     Err(mpu6050::Mpu6050Error::Spi(_)) => {
    //         // SPI通信错误
    //     }
    //     Err(mpu6050::Mpu6050Error::DeviceNotFound) => {
    //         // 设备未找到
    //     }
    //     Err(e) => {
    //         // 其他错误
    //         return Err(e);
    //     }
    // }
    
    Ok(())
}

/// 数值类型转换示例
fn example_numeric_conversions() {
    // f32和定点数之间的转换
    let f32_val = 3.14f32;
    let fixed_val = FixedI16F16::from_f32(f32_val);
    let back_to_f32 = fixed_val.to_f32();
    
    // 数学运算
    let a = FixedI16F16::from_f32(2.0);
    let b = FixedI16F16::from_f32(3.0);
    let sum = a + b;
    let product = a * b;
    let sqrt_a = a.sqrt();
    
    // 角度转换
    let degrees = f32::from_f32(90.0);
    let radians = degrees * f32::deg_to_rad();
    
    // 定点数版本
    let degrees_fixed = FixedI16F16::from_f32(90.0);
    let radians_fixed = degrees_fixed * FixedI16F16::deg_to_rad();
}

/// 配置预设示例
fn example_config_presets() {
    // 高精度配置（低噪声，高分辨率）
    let _high_precision = NewConfigBuilder::high_precision()
        .build()
        .unwrap();
    
    // 高速配置（高采样率）
    let _high_speed = NewConfigBuilder::high_speed()
        .build()
        .unwrap();
    
    // 低功耗配置
    let _low_power = NewConfigBuilder::low_power()
        .build()
        .unwrap();
    
    // 运动检测配置
    let _motion_detection = NewConfigBuilder::motion_detection()
        .build()
        .unwrap();
    
    // 姿态估计配置
    let _attitude_estimation = NewConfigBuilder::attitude_estimation()
        .build()
        .unwrap();
}

// 注意：这个示例文件不会被编译为可执行文件
// 它只是展示API的使用方法
// 在实际项目中，你需要：
// 1. 替换注释掉的代码为实际的HAL类型和操作
// 2. 添加适当的main函数和执行器
// 3. 根据具体硬件平台调整配置
// 4. 处理实际的错误情况

/// 向后兼容性示例
/// 
/// 如果你有现有的代码使用旧的API，可以这样迁移：
fn example_backward_compatibility() {
    // 旧的方式（仍然支持）：
    // let config = ConfigBuilder::default()
    //     .accel_scale(AccelScale::Scale2G)
    //     .build();
    
    // 新的方式（推荐）：
    let _config = NewConfigBuilder::new()
        .accel_scale(mpu6050::config::AccelScale::Scale2G)
        .build()
        .unwrap();
    
    // 类型别名提供向后兼容性：
    // type OldMpu6050<SPI, CS> = mpu6050::Mpu6050F32<SPI, CS>;
    // type OldSensorData = mpu6050::SensorDataF32;
}
