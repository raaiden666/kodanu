use crate::{Schedule, Stage, SystemContext, schedule::System};

#[derive(Default)]
pub struct Scheduler {
    startup: Schedule,
    pre_update: Schedule,
    update: Schedule,
    late_update: Schedule,
}

impl Scheduler {
    #[inline]
    pub fn run(&mut self, stage: Stage, context: &mut SystemContext) {
        match stage {
            Stage::PreUpdate => &mut self.pre_update.run(context),
            Stage::Startup => &mut self.startup.run(context),
            Stage::Update => &mut self.update.run(context),
            Stage::LateUpdate => &mut self.late_update.run(context),
        };
    }

    pub fn add(&mut self, stage: Stage, system: System) {
        match stage {
            Stage::PreUpdate => &mut self.pre_update.add(system),
            Stage::Startup => &mut self.startup.add(system),
            Stage::Update => &mut self.update.add(system),
            Stage::LateUpdate => &mut self.late_update.add(system),
        };
    }
}
