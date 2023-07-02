use std::ffi::c_int;

use crate::FsmBdd;
use sylvan::{Bdd, Sylvan};

extern "C" {
    pub fn lace_reachable(
        state: u64,
        trans: u64,
        invariants: u64,
        forward: c_int,
        contain_from: c_int,
        constrain: u64,
    ) -> u64;
}

impl FsmBdd<Sylvan> {
    pub fn lace_reachable_with_constrain(
        &self,
        state: &Bdd,
        forward: bool,
        contain_from: bool,
        constrain: &Bdd,
    ) -> Bdd {
        assert!(self.trans.trans.len() == 1);
        let res = unsafe {
            lace_reachable(
                state.get_raw(),
                self.trans.trans[0].get_raw(),
                self.invariants.get_raw(),
                forward as _,
                contain_from as _,
                constrain.get_raw(),
            )
        };
        unsafe { Bdd::new_from_raw(res) }
    }

    pub fn lace_reachable(&self, state: &Bdd, forward: bool, contain_from: bool) -> Bdd {
        self.lace_reachable_with_constrain(
            state,
            forward,
            contain_from,
            &self.manager.constant(true),
        )
    }

    pub fn lace_reachable_from_init(&self) -> Bdd {
        self.lace_reachable(&self.init, true, true)
    }

    pub fn lace_fair_cycle_with_constrain(&self, constrain: &Bdd) -> Bdd {
        let mut res = constrain.clone();
        let mut y = 0;
        loop {
            y += 1;
            dbg!(y);
            let mut new = res.clone();
            for fair in self.justice.iter() {
                let fair = fair & &res;
                let backward = self.lace_reachable_with_constrain(&fair, false, false, constrain);
                new &= backward;
            }
            if new == res {
                break res;
            }
            res = new
        }
    }

    pub fn lace_fair_cycle(&self) -> Bdd {
        self.lace_fair_cycle_with_constrain(&self.manager.constant(true))
    }
}
