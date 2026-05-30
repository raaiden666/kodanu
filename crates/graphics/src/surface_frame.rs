use wgpu::SurfaceTexture;

pub enum SurfaceFrame {
    Ready(SurfaceTexture),
    Suboptimal(SurfaceTexture),
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}
