use crate::res::{JUST_PRESSED_CAPACITY, JUST_RELEASED_CAPACITY, PRESSED_CAPACITY};

use std::{collections::HashSet, hash::Hash};

pub struct ButtonState<T>
where
    T: Eq + Hash + Copy,
{
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T> ButtonState<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            pressed: HashSet::with_capacity(PRESSED_CAPACITY),
            just_pressed: HashSet::with_capacity(JUST_PRESSED_CAPACITY),
            just_released: HashSet::with_capacity(JUST_RELEASED_CAPACITY),
        }
    }

    pub fn begin_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn press(&mut self, button: T) {
        if self.pressed.insert(button) {
            self.just_pressed.insert(button);
        }
    }

    pub fn release(&mut self, button: T) {
        if self.pressed.remove(&button) {
            self.just_released.insert(button);
        }
    }

    pub fn is_pressed(&self, button: T) -> bool {
        self.pressed.contains(&button)
    }

    pub fn is_just_pressed(&self, button: T) -> bool {
        self.just_pressed.contains(&button)
    }

    pub fn is_just_released(&self, button: T) -> bool {
        self.just_released.contains(&button)
    }
}
