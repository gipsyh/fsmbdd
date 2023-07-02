#include <sylvan.h>
#include <sylvan_mtbdd.h>
#include <lace.h>

TASK_6(BDD, reachable, BDD, state, BDD, trans, BDD, invariants, int, forward,
       int, contain_from, BDD, constrain)
{
	BDD tmp0 = sylvan_and(state, constrain);
	sylvan_protect(&tmp0);
	BDD frontier = sylvan_and(tmp0, invariants);
	sylvan_unprotect(&tmp0);
	sylvan_protect(&frontier);
	BDD reach;
	if (contain_from == 1) {
		reach = frontier;
	} else {
		reach = sylvan_false;
	}
	sylvan_protect(&reach);
	while (1) {
		BDD tmp1;
		if (forward == 1) {
			tmp1 = sylvan_relnext(frontier, trans, sylvan_false);
		} else {
			tmp1 = sylvan_relprev(trans, frontier, sylvan_false);
		}
		sylvan_protect(&tmp1);
		BDD tmp2 = sylvan_and(tmp1, constrain);
		sylvan_unprotect(&tmp1);
		sylvan_protect(&tmp2);
		BDD tmp3 = sylvan_and(tmp2, invariants);
		sylvan_unprotect(&tmp2);
		sylvan_protect(&tmp3);
		BDD new_frontier = sylvan_diff(tmp3, reach);
		sylvan_unprotect(&tmp3);
		sylvan_protect(&new_frontier);
		if (new_frontier == sylvan_false) {
			sylvan_unprotect(&new_frontier);
			sylvan_unprotect(&frontier);
			sylvan_unprotect(&reach);
			return reach;
		}
		BDD tmp4 = reach;
		BDD tmp5 = frontier;
		reach = sylvan_or(reach, new_frontier);
		frontier = new_frontier;
		sylvan_unprotect(&tmp4);
		sylvan_unprotect(&tmp5);
		sylvan_unprotect(&new_frontier);
	}
}

BDD lace_reachable(BDD state, BDD trans, BDD invariants, int forward,
		   int contain_from, BDD constrain)
{
	return RUN(reachable, state, trans, invariants, forward, contain_from, constrain);
}