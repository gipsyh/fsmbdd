use crate::{FsmBdd, Trans};
use sylvan::{lace_call_back, Bdd, LaceCallback, LaceWorkerContext, Sylvan};

struct LaceReachableFromInitCallbackArg<'a> {
    fsmbdd: &'a FsmBdd<Sylvan>,
}

pub struct LaceReachableFromInitCallback;

impl LaceCallback<LaceReachableFromInitCallbackArg<'_>, Bdd> for LaceReachableFromInitCallback {
    fn callback(
        _context: LaceWorkerContext,
        arg: &mut LaceReachableFromInitCallbackArg<'_>,
    ) -> Bdd {
        arg.fsmbdd.reachable_from_init()
    }
}

impl FsmBdd<Sylvan> {
    pub fn lace_reachable_from_init(&self) -> Bdd {
        let mut arg = LaceReachableFromInitCallbackArg { fsmbdd: self };
        lace_call_back::<LaceReachableFromInitCallback, LaceReachableFromInitCallbackArg<'_>, Bdd>(
            &mut arg,
        )
    }
}

struct LaceFairCycleWithConstrainCallbackArg<'a> {
    fsmbdd: &'a FsmBdd<Sylvan>,
    constrain: &'a Bdd,
}

pub struct LaceFairCycleWithConstrainCallback;

impl LaceCallback<LaceFairCycleWithConstrainCallbackArg<'_>, Bdd>
    for LaceFairCycleWithConstrainCallback
{
    fn callback(
        _context: LaceWorkerContext,
        arg: &mut LaceFairCycleWithConstrainCallbackArg<'_>,
    ) -> Bdd {
        arg.fsmbdd.fair_cycle_with_constrain(arg.constrain)
    }
}

impl FsmBdd<Sylvan> {
    pub fn lace_fair_cycle_with_constrain(&self, constrain: &Bdd) -> Bdd {
        let mut arg = LaceFairCycleWithConstrainCallbackArg {
            fsmbdd: self,
            constrain,
        };
        lace_call_back::<
            LaceFairCycleWithConstrainCallback,
            LaceFairCycleWithConstrainCallbackArg<'_>,
            Bdd,
        >(&mut arg)
    }
}

impl Trans<Sylvan> {
    pub fn lace_spawn_post_image(&self, context: &mut LaceWorkerContext, state: &Bdd) {
        if self.trans.len() == 1 {
            context.spawn_post_image(state, &self.trans[0]);
        } else {
            todo!()
        }
    }

    pub fn lace_sync_post_image(&self, context: &mut LaceWorkerContext) -> Bdd {
        if self.trans.len() == 1 {
            context.sync_post_image()
        } else {
            todo!()
        }
    }

    pub fn lace_spawn_pre_image(&self, context: &mut LaceWorkerContext, state: &Bdd) {
        if self.trans.len() == 1 {
            context.spawn_pre_image(state, &self.trans[0]);
        } else {
            todo!()
        }
    }

    pub fn lace_sync_pre_image(&self, context: &mut LaceWorkerContext) -> Bdd {
        if self.trans.len() == 1 {
            context.sync_pre_image()
        } else {
            todo!()
        }
    }
}

impl FsmBdd<Sylvan> {
    pub fn lace_spawn_post_image(&self, context: &mut LaceWorkerContext, state: &Bdd) {
        self.trans
            .lace_spawn_post_image(context, &(state & &self.invariants));
    }

    pub fn lace_sync_post_image(&self, context: &mut LaceWorkerContext) -> Bdd {
        self.trans.lace_sync_post_image(context) & &self.invariants
    }

    pub fn lace_spawn_pre_image(&self, context: &mut LaceWorkerContext, state: &Bdd) {
        self.trans
            .lace_spawn_pre_image(context, &(state & &self.invariants));
    }

    pub fn lace_sync_pre_image(&self, context: &mut LaceWorkerContext) -> Bdd {
        self.trans.lace_sync_pre_image(context) & &self.invariants
    }
}
