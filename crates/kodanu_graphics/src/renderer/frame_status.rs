pub enum FrameStatus {
    Success,
    Suboptimal,
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}

impl FrameStatus {
    #[inline]
    pub fn requires_surface_recovery(&self) -> bool {
        matches!(
            self,
            FrameStatus::Suboptimal | FrameStatus::Outdated | FrameStatus::Lost
        )
    }

    #[inline]
    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::Validation)
    }
}
