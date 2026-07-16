use {kodanu_editor::Scene, kodanu_input::Input, kodanu_time::Time};

pub struct SystemContext<'s> {
    pub scene: &'s mut Scene,
    pub time: &'s Time,
    pub input: &'s Input,
}
