use crate::{trans::Trans, FsmBdd};
use bdds::BddManager;
use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor, Not},
};

#[derive(Clone, Debug)]
pub struct PartitionedFsmBdd<BM: BddManager>
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    pub symbols: HashMap<String, usize>,
    pub manager: BM,
    pub slice: Vec<BM::Bdd>,
    pub init: Vec<BM::Bdd>,
    pub trans: Trans<BM>,
    pub fair: Vec<BM::Bdd>,
}

impl<BM: BddManager> PartitionedFsmBdd<BM>
where
    for<'a, 'b> &'a BM::Bdd: Not<Output = BM::Bdd>
        + BitAnd<BM::Bdd, Output = BM::Bdd>
        + BitAnd<&'b BM::Bdd, Output = BM::Bdd>
        + BitOr<BM::Bdd, Output = BM::Bdd>
        + BitOr<&'b BM::Bdd, Output = BM::Bdd>
        + BitXor<BM::Bdd, Output = BM::Bdd>
        + BitXor<&'b BM::Bdd, Output = BM::Bdd>,
{
    fn slice_bdd(bdd: &BM::Bdd, slice: &[BM::Bdd]) -> Vec<BM::Bdd> {
        let mut res = Vec::new();
        for slice in slice.iter() {
            res.push(bdd & slice);
        }
        res
    }

    fn reslice_bdd(&self, bdd: &[BM::Bdd]) -> Vec<BM::Bdd> {
        let mut res = vec![self.manager.constant(false); self.slice.len()];
        for i in 0..self.slice.len() {
            for j in 0..bdd.len() {
                res[i] |= &bdd[j] & &self.slice[i];
            }
        }
        res
    }

    pub fn new(fsmbdd: &FsmBdd<BM>, partition: &[usize]) -> Self {
        assert!(partition.len() > 0);
        let mut slice = Vec::new();
        for i in 0..(1 << partition.len()) {
            let mut res = fsmbdd.manager.constant(true);
            for j in 0..partition.len() {
                res &= if i & (1 << j) > 0 {
                    fsmbdd.manager.ith_var(partition[j] * 2)
                } else {
                    !fsmbdd.manager.ith_var(partition[j] * 2)
                }
            }
            slice.push(res)
        }
        let init = Self::slice_bdd(&fsmbdd.init, &slice);
        let fair = Self::slice_bdd(&fsmbdd.fair, &slice);
        Self {
            symbols: fsmbdd.symbols.clone(),
            manager: fsmbdd.manager.clone(),
            slice,
            init,
            trans: fsmbdd.trans.clone(),
            fair,
        }
    }

    pub fn reachable_with_constrain(
        &self,
        state: &[BM::Bdd],
        forward: bool,
        contain_from: bool,
        constrain: &[BM::Bdd],
    ) -> Vec<BM::Bdd> {
        assert!(state.len() == self.slice.len());
        assert!(constrain.len() == self.slice.len());
        let mut frontier = state.to_vec();
        for i in 0..frontier.len() {
            frontier[i] &= &constrain[i]
        }
        let mut reach = if contain_from {
            frontier.clone()
        } else {
            vec![self.manager.constant(false); self.slice.len()]
        };
        let mut x = 0;
        loop {
            x += 1;
            dbg!(x);
            let new_frontier: Vec<BM::Bdd> = frontier
                .iter()
                .map(|bdd| {
                    if forward {
                        self.trans.post_image(bdd)
                    } else {
                        self.trans.pre_image(bdd)
                    }
                })
                .collect();
            let mut new_frontier = self.reslice_bdd(&new_frontier);
            for i in 0..new_frontier.len() {
                new_frontier[i] &= &constrain[i];
                new_frontier[i] &= !&reach[i];
            }
            if new_frontier
                .iter()
                .all(|bdd| *bdd == self.manager.constant(false))
            {
                break reach;
            }
            for i in 0..reach.len() {
                reach[i] |= &new_frontier[i];
            }
            frontier = new_frontier;
        }
    }

    pub fn reachable(&self, state: &[BM::Bdd], forward: bool, contain_from: bool) -> Vec<BM::Bdd> {
        self.reachable_with_constrain(
            state,
            forward,
            contain_from,
            &vec![self.manager.constant(true); self.slice.len()],
        )
    }

    pub fn reachable_from_init(&self) -> Vec<BM::Bdd> {
        self.reachable(&self.init, true, true)
    }

    pub fn fair_cycle_with_constrain(&self, constrain: &[BM::Bdd]) -> Vec<BM::Bdd> {
        let mut res = self.fair.clone();
        for i in 0..res.len() {
            res[i] &= &constrain[i]
        }
        let mut y = 0;
        loop {
            y += 1;
            dbg!(y);
            let backward = self.reachable_with_constrain(&res, false, false, constrain);
            let mut new = Vec::new();
            for i in 0..backward.len() {
                new.push(&res[i] & &backward[i])
            }
            if new == res {
                break res;
            }
            res = new
        }
    }
}
