//! 数值类型抽象模块
//!
//! 提供统一的数值类型接口，支持浮点数和定点数运算

use core::fmt::Debug;

/// 数值类型trait，定义了MPU6050库中需要的所有数值运算
pub trait NumericType:
    Copy
    + Clone
    + Debug
    + PartialEq
    + PartialOrd
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::Neg<Output = Self>
{
    /// 零值常量
    fn zero() -> Self;

    /// 单位值常量
    fn one() -> Self;

    /// 从i16原始值转换
    fn from_raw_i16(raw: i16) -> Self;

    /// 从f32转换
    fn from_f32(val: f32) -> Self;

    /// 转换为f32
    fn to_f32(self) -> f32;

    /// 平方根
    fn sqrt(self) -> Self;

    /// 反正切函数 atan2
    fn atan2(y: Self, x: Self) -> Self;

    /// 幂运算
    fn powf(self, exp: Self) -> Self;

    /// 绝对值
    fn abs(self) -> Self;

    /// 常量：PI
    fn pi() -> Self;

    /// 常量：角度到弧度转换因子
    fn deg_to_rad() -> Self {
        Self::pi() / Self::from_f32(180.0)
    }

    /// 常量：弧度到角度转换因子
    fn rad_to_deg() -> Self {
        Self::from_f32(180.0) / Self::pi()
    }
}

/// f32的NumericType实现
impl NumericType for f32 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn one() -> Self {
        1.0
    }

    #[inline]
    fn from_raw_i16(raw: i16) -> Self {
        raw as f32
    }

    #[inline]
    fn from_f32(val: f32) -> Self {
        val
    }

    #[inline]
    fn to_f32(self) -> f32 {
        self
    }

    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    #[inline]
    fn atan2(y: Self, x: Self) -> Self {
        libm::atan2f(y, x)
    }

    #[inline]
    fn powf(self, exp: Self) -> Self {
        libm::powf(self, exp)
    }

    #[inline]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }

    #[inline]
    fn pi() -> Self {
        core::f32::consts::PI
    }
}

/// f64的NumericType实现
impl NumericType for f64 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn one() -> Self {
        1.0
    }

    #[inline]
    fn from_raw_i16(raw: i16) -> Self {
        raw as f64
    }

    #[inline]
    fn from_f32(val: f32) -> Self {
        val as f64
    }

    #[inline]
    fn to_f32(self) -> f32 {
        self as f32
    }

    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    #[inline]
    fn atan2(y: Self, x: Self) -> Self {
        libm::atan2(y, x)
    }

    #[inline]
    fn powf(self, exp: Self) -> Self {
        libm::pow(self, exp)
    }

    #[inline]
    fn abs(self) -> Self {
        libm::fabs(self)
    }

    #[inline]
    fn pi() -> Self {
        core::f64::consts::PI
    }
}

// 定点数类型别名
pub type FixedI16F16 = fixed::FixedI32<fixed::types::extra::U16>;
pub type FixedI8F24 = fixed::FixedI32<fixed::types::extra::U24>;

/// 定点数的NumericType实现（I16F16格式）
impl NumericType for FixedI16F16 {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn from_raw_i16(raw: i16) -> Self {
        Self::from_num(raw)
    }

    #[inline]
    fn from_f32(val: f32) -> Self {
        Self::from_num(val)
    }

    #[inline]
    fn to_f32(self) -> f32 {
        self.to_num()
    }

    #[inline]
    fn sqrt(self) -> Self {
        // 使用牛顿法近似计算平方根
        if self <= Self::ZERO {
            return Self::ZERO;
        }

        let mut x = self;
        let mut prev;

        // 牛顿迭代法: x_{n+1} = (x_n + a/x_n) / 2
        for _ in 0..10 {
            prev = x;
            x = (x + self / x) / Self::from_num(2);

            // 检查收敛
            if (x - prev).abs() < Self::from_num(0.001) {
                break;
            }
        }

        x
    }

    #[inline]
    fn atan2(y: Self, x: Self) -> Self {
        // 简化的atan2实现，使用查找表或多项式近似
        let y_f = y.to_f32();
        let x_f = x.to_f32();
        Self::from_f32(libm::atan2f(y_f, x_f))
    }

    #[inline]
    fn powf(self, exp: Self) -> Self {
        // 对于定点数，使用浮点数计算然后转换回来
        let base_f = self.to_f32();
        let exp_f = exp.to_f32();
        Self::from_f32(libm::powf(base_f, exp_f))
    }

    #[inline]
    fn abs(self) -> Self {
        if self < Self::ZERO { -self } else { self }
    }

    #[inline]
    fn pi() -> Self {
        Self::from_num(core::f32::consts::PI)
    }
}

/// 数值类型转换工具
pub struct NumericConverter;

impl NumericConverter {
    /// 将原始传感器数据转换为指定数值类型
    pub fn convert_sensor_data<T: NumericType>(
        raw: (i16, i16, i16),
        scale_factor: f32,
        offset: (i16, i16, i16),
    ) -> (T, T, T) {
        let x = T::from_f32((raw.0 - offset.0) as f32 / scale_factor);
        let y = T::from_f32((raw.1 - offset.1) as f32 / scale_factor);
        let z = T::from_f32((raw.2 - offset.2) as f32 / scale_factor);
        (x, y, z)
    }

    /// 角度转弧度
    pub fn deg_to_rad<T: NumericType>(deg: T) -> T {
        deg * T::deg_to_rad()
    }

    /// 弧度转角度
    pub fn rad_to_deg<T: NumericType>(rad: T) -> T {
        rad * T::rad_to_deg()
    }
}

// 测试模块将在后续添加到单独的测试文件中
