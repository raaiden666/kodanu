use crate::{Schedule, Stage, SystemContext, schedule::System};

#[derive(Default)]
pub struct Scheduler {
    startup: Schedule,
    begin: Schedule,
    update: Schedule,
    render: Schedule,
}

impl Scheduler {
    #[inline]
    fn stage_mut(&mut self, stage: Stage) -> &mut Schedule {
        match stage {
            Stage::Begin => &mut self.begin,
            Stage::Startup => &mut self.startup,
            Stage::Update => &mut self.update,
            Stage::Render => &mut self.render,
        }
    }

    #[inline]
    pub fn add(&mut self, stage: Stage, system: System) {
        self.stage_mut(stage).add(system);
    }

    #[inline]
    pub fn adds(&mut self, stage: Stage, systems: impl IntoIterator<Item = System>) {
        self.stage_mut(stage).adds(systems);
    }

    #[inline]
    pub fn startup(&mut self, context: &mut SystemContext) {
        self.startup.run(context);
    }

    #[inline]
    pub fn update(&mut self, context: &mut SystemContext) {
        self.update.run(context);
    }

    #[inline]
    pub fn render(&mut self, context: &mut SystemContext) {
        self.render.run(context);
    }
}
