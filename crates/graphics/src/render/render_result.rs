pub enum RenderResult {
    Success,
    Suboptimal,
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
}

impl RenderResult {
    pub fn requires_surface_recovery(&self) -> bool {
        matches!(
            self,
            RenderResult::Suboptimal | RenderResult::Outdated | RenderResult::Lost
        )
    }

    pub fn is_fatal(&self) -> bool {
        matches!(self, Self::Validation)
    }
}
