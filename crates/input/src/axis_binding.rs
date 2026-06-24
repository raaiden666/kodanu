use crate::KeyCode;

pub(crate) struct AxisBinding {
    positive: KeyCode,
    negative: KeyCode,
}

impl AxisBinding {
    pub fn new(positive: KeyCode, negative: KeyCode) -> Self {
        Self { positive, negative }
    }
}

impl AxisBinding {
    pub fn positive(&self) -> KeyCode {
        self.positive
    }

    pub fn negative(&self) -> KeyCode {
        self.negative
    }
}
