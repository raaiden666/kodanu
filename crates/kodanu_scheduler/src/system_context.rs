use {
    kodanu_editor::Scene,
    kodanu_input::{ActionMap, Input},
    kodanu_time::Time,
};

pub struct SystemContext<'s> {
    pub scene: &'s mut Scene,
    pub time: &'s Time,
    pub input: &'s Input,
    pub action_map: &'s ActionMap,
}
