use wgpu::SurfaceTexture;

pub(crate) enum SurfaceFrame {
    Ready(SurfaceTexture),
    Suboptimal(SurfaceTexture),
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}
