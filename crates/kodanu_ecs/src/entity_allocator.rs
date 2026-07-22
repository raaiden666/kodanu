use crate::entity::Entity;

#[derive(Default)]
pub struct EntityAllocator {
    generations: Vec<u32>,
    free_indicies: Vec<u32>,
}

impl EntityAllocator {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            generations: Vec::with_capacity(capacity),
            free_indicies: Vec::with_capacity(capacity),
        }
    }

    pub fn create(&mut self) -> Entity {
        if let Some(index) = self.free_indicies.pop() {
            Entity {
                index,
                generation: self.generations[index as usize],
            }
        } else {
            let index = self.generations.len() as u32;

            self.generations.push(0);

            Entity {
                index,
                generation: 0,
            }
        }
    }

    pub fn destroy(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        self.generations[entity.index as usize] += 1;
        self.free_indicies.push(entity.index);

        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations
            .get(entity.index as usize)
            .is_some_and(|generation| *generation == entity.generation)
    }
}
