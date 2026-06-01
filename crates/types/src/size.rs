use winit::dpi::{PhysicalSize as WinitPhysicalSize, Size as WinitSize};

#[derive(Debug, Clone, Copy)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl Size<u32> {
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl From<WinitPhysicalSize<u32>> for Size<u32> {
    fn from(size: WinitPhysicalSize<u32>) -> Self {
        Self::clamped(size.width, size.height)
    }
}

impl From<Size<u32>> for WinitSize {
    fn from(size: Size<u32>) -> Self {
        WinitSize::Physical(WinitPhysicalSize::new(size.width, size.height))
    }
}
