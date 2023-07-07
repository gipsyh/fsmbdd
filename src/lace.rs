use crate::{FsmBdd, Trans};
use sylvan::{Bdd, LaceWorkerContext, Sylvan};

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
        let trans = self.trans.clone();
        let state = state.clone();
        let invariants = self.invariants.clone();
        context.lace_spawn(move |_| trans.post_image(&state) & invariants);
    }

    pub fn lace_sync_post_image(&self, context: &mut LaceWorkerContext) -> Bdd {
        context.lace_sync::<Bdd>()
    }

    pub fn lace_spawn_pre_image(&self, context: &mut LaceWorkerContext, state: &Bdd) {
        let trans = self.trans.clone();
        let state = state.clone();
        let invariants = self.invariants.clone();
        context.lace_spawn(move |_| trans.pre_image(&state) & invariants);
    }

    pub fn lace_sync_pre_image(&self, context: &mut LaceWorkerContext) -> Bdd {
        context.lace_sync::<Bdd>()
    }
}
