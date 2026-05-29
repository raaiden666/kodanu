use winit::dpi::{PhysicalSize, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizeU32 {
    pub width: u32,
    pub height: u32,
}

impl SizeU32 {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl From<PhysicalSize<u32>> for SizeU32 {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self::new(size.width, size.height)
    }
}

impl From<SizeU32> for PhysicalSize<u32> {
    fn from(size: SizeU32) -> Self {
        Self::new(size.width, size.height)
    }
}

impl From<SizeU32> for Size {
    fn from(size: SizeU32) -> Self {
        Size::Physical(PhysicalSize::new(size.width, size.height))
    }
}
