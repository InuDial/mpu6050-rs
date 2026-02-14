# mpu6050

高效、易用的 MPU6050 六轴传感器 Rust 驱动库，支持 async/await，适用于嵌入式平台。

## 特性

- 🚀 支持 async/await，适配 Embassy/RTIC 等异步框架
- 🔢 **泛型数值类型支持**：同时支持浮点数（f32/f64）和定点数运算
- ⚙️ 支持自定义加速度计/陀螺仪量程、DLPF、采样率等
- 🎯 支持加速度计/陀螺仪校准
- 📊 支持 FIFO、中断等高级功能
- 🔧 兼容 [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits
- 🚫 无需 std，适合 no_std 环境
- 🛡️ 类型安全的配置系统
- 📝 统一的错误处理机制

## 快速开始

### 基本使用（浮点数）

```rust
use mpu6050::{Mpu6050, config::NewConfigBuilder, SensorData};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // 初始化 SPI/CS（请替换为你的外设）
    let spi = ...;
    let cs = ...;

    // 使用新的配置构建器
    let config = NewConfigBuilder::high_precision()
        .sample_rate(200)
        .enable_interrupts(true)
        .build()
        .unwrap();

    // 创建MPU6050实例（默认使用f32）
    let mut mpu: Mpu6050<_, _, f32> = Mpu6050::new(spi, cs, config);

    mpu.init_with_config().await.unwrap();
    mpu.calibrate_sensors(1000).await.unwrap();

    loop {
        let data: SensorData<f32> = mpu.read_all().await.unwrap();
        // 处理 data.accel, data.gyro, data.temp
    }
}
```

### 定点数支持（资源受限系统）

```rust
use mpu6050::{Mpu6050, config::NewConfigBuilder, SensorData, FixedI16F16};

// 使用定点数类型，节省内存和计算资源
let config = NewConfigBuilder::low_power()
    .sample_rate(50)
    .build()
    .unwrap();

let mut mpu: Mpu6050<_, _, FixedI16F16> = Mpu6050::new(spi, cs, config);

let data: SensorData<FixedI16F16> = mpu.read_all().await.unwrap();
let accel_magnitude = {
    let (ax, ay, az) = data.accel;
    (ax * ax + ay * ay + az * az).sqrt()
};

// 转换为浮点数进行显示
let magnitude_f32 = accel_magnitude.to_f32();
```

### 配置预设

库提供了多种预设配置，适用于不同的应用场景：

```rust
use mpu6050::config::NewConfigBuilder;

// 高精度配置（低噪声，高分辨率）
let high_precision = NewConfigBuilder::high_precision().build().unwrap();

// 高速配置（高采样率）
let high_speed = NewConfigBuilder::high_speed().build().unwrap();

// 低功耗配置
let low_power = NewConfigBuilder::low_power().build().unwrap();

// 运动检测配置
let motion_detection = NewConfigBuilder::motion_detection().build().unwrap();

// 姿态估计配置
let attitude_estimation = NewConfigBuilder::attitude_estimation().build().unwrap();
```

### 自定义配置

```rust
use mpu6050::config::{NewConfigBuilder, AccelScale, GyroScale, DlpfConfig};

let config = NewConfigBuilder::new()
    .accel_scale(AccelScale::Scale4G)
    .gyro_scale(GyroScale::Scale500)
    .dlpf_config(DlpfConfig::Bandwidth20Hz)
    .sample_rate(100)
    .enable_fifo(true)
    .enable_interrupts(true)
    .build()
    .unwrap();
```

## 数值类型支持

### 支持的数值类型

- **f32**: 标准 32 位浮点数，适用于有 FPU 的系统
- **f64**: 64 位浮点数，更高精度
- **FixedI16F16**: 32 位定点数（16 位整数部分 + 16 位小数部分）
- **FixedI8F24**: 32 位定点数（8 位整数部分 + 24 位小数部分）

### 数值类型转换

```rust
use mpu6050::{NumericType, FixedI16F16};

// 从原始传感器数据转换
let raw_value: i16 = 1000;
let f32_value = f32::from_raw_i16(raw_value);
let fixed_value = FixedI16F16::from_raw_i16(raw_value);

// 类型间转换
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
let degrees = 90.0f32;
let radians = degrees * f32::deg_to_rad();
```

## 错误处理

库提供了统一的错误处理机制：

```rust
use mpu6050::{Mpu6050Error, Result};

// 使用Result类型
let config = NewConfigBuilder::new()
    .sample_rate(100)
    .build()?; // 自动处理配置错误

// 匹配具体错误类型
match mpu.init_with_config().await {
    Ok(_) => println!("初始化成功"),
    Err(Mpu6050Error::Spi(e)) => println!("SPI通信错误: {:?}", e),
    Err(Mpu6050Error::DeviceNotFound) => println!("设备未找到"),
    Err(Mpu6050Error::InvalidConfig) => println!("配置无效"),
    Err(e) => println!("其他错误: {:?}", e),
}
```

## 向后兼容性

为了保持向后兼容性，库提供了类型别名：

```rust
// 新的泛型版本
use mpu6050::{Mpu6050, SensorData};
let mpu: Mpu6050<_, _, f32> = Mpu6050::new(spi, cs, config);
let data: SensorData<f32> = mpu.read_all().await?;

// 向后兼容的别名
use mpu6050::{Mpu6050F32, SensorDataF32};
let mpu: Mpu6050F32<_, _> = Mpu6050::new(spi, cs, config);
let data: SensorDataF32 = mpu.read_all().await?;
```

## 性能对比

### 浮点数 vs 定点数

| 特性     | f32             | FixedI16F16     | 适用场景             |
| -------- | --------------- | --------------- | -------------------- |
| 内存使用 | 4 字节          | 4 字节          | 相同                 |
| 计算速度 | 快（有 FPU 时） | 快（无 FPU 时） | 取决于硬件           |
| 精度     | 高              | 中等            | 科学计算 vs 控制系统 |
| 功耗     | 中等            | 低              | 电池供电系统         |

### 配置预设性能

| 预设                | 采样率 | 功耗 | 精度 | 适用场景 |
| ------------------- | ------ | ---- | ---- | -------- |
| high_precision      | 100Hz  | 中等 | 最高 | 科学测量 |
| high_speed          | 1000Hz | 高   | 高   | 实时控制 |
| low_power           | 50Hz   | 最低 | 中等 | 电池设备 |
| motion_detection    | 200Hz  | 中等 | 高   | 运动感知 |
| attitude_estimation | 200Hz  | 中等 | 高   | 姿态解算 |

## API 说明

### 配置相关

- `NewConfigBuilder`：新的类型安全配置构建器
- `NewConfigBuilder::high_precision()`：高精度预设配置
- `NewConfigBuilder::high_speed()`：高速预设配置
- `NewConfigBuilder::low_power()`：低功耗预设配置
- `NewConfigBuilder::motion_detection()`：运动检测预设配置
- `NewConfigBuilder::attitude_estimation()`：姿态估计预设配置

### 设备操作

- `Mpu6050::new()`：创建 MPU6050 实例（支持泛型数值类型）
- `Mpu6050::init_with_config()`：初始化并写入配置
- `Mpu6050::calibrate_sensors()`：校准加速度计和陀螺仪
- `Mpu6050::who_am_i()`：读取设备 ID

### 数据读取

- `Mpu6050::read_all()`：读取所有传感器数据（泛型版本）
- `Mpu6050::read_accel()`：读取加速度计数据
- `Mpu6050::read_gyro()`：读取陀螺仪数据
- `Mpu6050::read_temp()`：读取温度数据

### 高级功能

- `Mpu6050::enable_fifo()` / `read_fifo_data()`：FIFO 操作
- `Mpu6050::enable_interrupts()` / `read_interrupt_status()`：中断操作
- `Mpu6050::calculate_pitch_roll_from_accel()`：基于加速度计的姿态计算

### 数值类型

- `NumericType` trait：统一的数值类型接口
- `FixedI16F16`、`FixedI8F24`：定点数类型
- `NumericConverter`：数值转换工具

更多详细信息见 [API 文档](https://docs.rs/mpu6050)

## 依赖与兼容性

- 依赖：`embedded-hal`、`embedded-hal-async`
- 兼容：STM32/ESP32/nRF52 等支持 async/await 的平台

## 许可证

MIT OR Apache-2.0

---
