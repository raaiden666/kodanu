use std::{collections::HashSet, hash::Hash};

pub(crate) struct ButtonState<T>
where
    T: Eq + Hash + Copy,
{
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T> Default for ButtonState<T>
where
    T: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self {
            pressed: HashSet::with_capacity(128),
            just_pressed: HashSet::with_capacity(128),
            just_released: HashSet::with_capacity(128),
        }
    }
}

impl<T> ButtonState<T>
where
    T: Eq + Hash + Copy,
{
    #[inline]
    pub fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    #[inline]
    pub fn press(&mut self, button: T) {
        if self.pressed.insert(button) {
            self.just_pressed.insert(button);
        }
    }

    #[inline]
    pub fn release(&mut self, button: T) {
        if self.pressed.remove(&button) {
            self.just_released.insert(button);
        }
    }

    #[inline]
    pub fn is_pressed(&self, button: T) -> bool {
        self.pressed.contains(&button)
    }

    #[inline]
    pub fn is_just_pressed(&self, button: T) -> bool {
        self.just_pressed.contains(&button)
    }

    #[inline]
    pub fn is_just_released(&self, button: T) -> bool {
        self.just_released.contains(&button)
    }
}
