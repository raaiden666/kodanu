use crate::SystemContext;

pub type System = fn(&mut SystemContext);

pub struct Schedule {
    systems: Vec<System>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            systems: Vec::with_capacity(1_000),
        }
    }
}

impl Schedule {
    #[inline]
    pub fn add(&mut self, system: System) {
        self.systems.push(system);
    }

    #[inline]
    pub fn adds<I>(&mut self, systems: I)
    where
        I: IntoIterator<Item = System>,
    {
        for system in systems {
            self.systems.push(system);
        }
    }

    #[inline]
    pub fn run(&mut self, context: &mut SystemContext) {
        for system in &self.systems {
            system(context);
        }
    }
}
