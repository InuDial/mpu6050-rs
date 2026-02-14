#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

const ALPHA: f32 = 0.98;

// ===== 设备识别相关寄存器 =====
/// 设备ID寄存器地址
pub const WHO_AM_I: u8 = 0x75;
/// 设备ID期望值
pub const WHO_AM_I_VALUE: u8 = 0x68;

// ===== 电源管理相关寄存器 =====
/// 电源管理1寄存器
pub const PWR_MGMT_1: u8 = 0x6B;
/// 电源管理2寄存器
pub const PWR_MGMT_2: u8 = 0x6C;

// ===== 配置相关寄存器 =====
/// 配置寄存器
pub const CONFIG: u8 = 0x1A;
/// 陀螺仪配置寄存器
pub const GYRO_CONFIG: u8 = 0x1B;
/// 加速度计配置寄存器
pub const ACCEL_CONFIG: u8 = 0x1C;
/// 加速度计配置2寄存器
// pub const ACCEL_CONFIG2: u8 = 0x1D;

// ===== 采样率相关寄存器 =====
/// 采样率分频寄存器
pub const SMPLRT_DIV: u8 = 0x19;

// ===== 中断相关寄存器 =====
/// 中断使能寄存器
pub const INT_ENABLE: u8 = 0x38;
/// 中断状态寄存器
pub const INT_STATUS: u8 = 0x3A;

// ===== 数据输出相关寄存器 =====
/// 加速度计X高字节
pub const ACCEL_XOUT_H: u8 = 0x3B;
/// 加速度计X低字节
pub const ACCEL_XOUT_L: u8 = 0x3C;
/// 加速度计Y高字节
pub const ACCEL_YOUT_H: u8 = 0x3D;
/// 加速度计Y低字节
pub const ACCEL_YOUT_L: u8 = 0x3E;
/// 加速度计Z高字节
pub const ACCEL_ZOUT_H: u8 = 0x3F;
/// 加速度计Z低字节
pub const ACCEL_ZOUT_L: u8 = 0x40;
/// 温度高字节
pub const TEMP_OUT_H: u8 = 0x41;
/// 温度低字节
pub const TEMP_OUT_L: u8 = 0x42;
/// 陀螺仪X高字节
pub const GYRO_XOUT_H: u8 = 0x43;
/// 陀螺仪X低字节
pub const GYRO_XOUT_L: u8 = 0x44;
/// 陀螺仪Y高字节
pub const GYRO_YOUT_H: u8 = 0x45;
/// 陀螺仪Y低字节
pub const GYRO_YOUT_L: u8 = 0x46;
/// 陀螺仪Z高字节
pub const GYRO_ZOUT_H: u8 = 0x47;
/// 陀螺仪Z低字节
pub const GYRO_ZOUT_L: u8 = 0x48;

// ===== 用户控制相关寄存器 =====
/// 用户控制寄存器
pub const USER_CTRL: u8 = 0x6A;

// ===== FIFO相关寄存器 =====
/// FIFO使能寄存器
pub const FIFO_EN: u8 = 0x23;
/// FIFO计数高字节
pub const FIFO_COUNTH: u8 = 0x72;
/// FIFO计数低字节
pub const FIFO_COUNTL: u8 = 0x73;
/// FIFO读写寄存器
pub const FIFO_R_W: u8 = 0x74;

// ===== 自检相关寄存器 =====
/// X轴加速度计自检
pub const SELF_TEST_X_ACCEL: u8 = 0x0D;
/// Y轴加速度计自检
pub const SELF_TEST_Y_ACCEL: u8 = 0x0E;
/// Z轴加速度计自检
pub const SELF_TEST_Z_ACCEL: u8 = 0x0F;
/// X轴陀螺仪自检
pub const SELF_TEST_X_GYRO: u8 = 0x00;
/// Y轴陀螺仪自检
pub const SELF_TEST_Y_GYRO: u8 = 0x01;
/// Z轴陀螺仪自检
pub const SELF_TEST_Z_GYRO: u8 = 0x02;

// ===== 偏移相关寄存器 =====
/// X轴加速度计偏移高字节
pub const XA_OFFSET_H: u8 = 0x77;
/// X轴加速度计偏移低字节
pub const XA_OFFSET_L: u8 = 0x78;
/// Y轴加速度计偏移高字节
pub const YA_OFFSET_H: u8 = 0x7A;
/// Y轴加速度计偏移低字节
pub const YA_OFFSET_L: u8 = 0x7B;
/// Z轴加速度计偏移高字节
pub const ZA_OFFSET_H: u8 = 0x7D;
/// Z轴加速度计偏移低字节
pub const ZA_OFFSET_L: u8 = 0x7E;

// ===== 物理常量定义 =====
/// 加速度计量程常量（±2g）
pub const ACCEL_SCALE_2G: f32 = 16384.0;
/// 加速度计量程常量（±4g）
pub const ACCEL_SCALE_4G: f32 = 8192.0;
/// 加速度计量程常量（±8g）
pub const ACCEL_SCALE_8G: f32 = 4096.0;
/// 加速度计量程常量（±16g）
pub const ACCEL_SCALE_16G: f32 = 2048.0;

/// 陀螺仪量程常量（±250°/s）
pub const GYRO_SCALE_250: f32 = 131.0;
/// 陀螺仪量程常量（±500°/s）
pub const GYRO_SCALE_500: f32 = 65.5;
/// 陀螺仪量程常量（±1000°/s）
pub const GYRO_SCALE_1000: f32 = 32.8;
/// 陀螺仪量程常量（±2000°/s）
pub const GYRO_SCALE_2000: f32 = 16.4;

/// 温度转换比例
pub const TEMP_SCALE: f32 = 333.87;
/// 温度转换偏移
pub const TEMP_OFFSET: f32 = 21.0;
