//! An implementation of zig-zag encoding
//!

/// A trait for zig-zag encoding
pub trait ZigZag {
    /// Target type
    type Target;

    /// Encodes this type into Self::Target
    fn zigzag(&self) -> Self::Target;
}

/// A trait for zig-zag decoding
pub trait ZagZig {
    /// Target type
    type Target;

    /// Bit mask for decoding
    const MASK: Self::Target;

    /// Decode current value into Self::Target
    fn zagzig(&self) -> Self::Target;
}

macro_rules! impl_zigzag {
    ($typ:ty, $target:ty) => {
        impl ZigZag for $typ {
            type Target = $target;
            fn zigzag(&self) -> Self::Target {
                let u = (self << 1) as $target;
                if *self < 0 {
                    1 ^ u
                } else {
                    u
                }
            }
        }
    };
}

macro_rules! impl_zagzig {
    ($typ:ty, $target:ty, $mask:expr) => {
        impl ZagZig for $typ {
            type Target = $target;
            const MASK: Self::Target = $mask;
            fn zagzig(&self) -> Self::Target {
                let i = (self >> 1) as $target;
                if self & 1 != 0 {
                    Self::MASK | i
                } else {
                    i
                }
            }
        }
    };
}

impl_zigzag!(i8, u8);
impl_zigzag!(i16, u16);
impl_zigzag!(i32, u32);
impl_zigzag!(i64, u64);
impl_zigzag!(i128, u128);

impl_zagzig!(u8, i8, -1 << 7);
impl_zagzig!(u16, i16, -1 << 15);
impl_zagzig!(u32, i32, -1 << 31);
impl_zagzig!(u64, i64, -1 << 63);
impl_zagzig!(u128, i128, -1 << 127);

#[cfg(test)]
mod tests;
