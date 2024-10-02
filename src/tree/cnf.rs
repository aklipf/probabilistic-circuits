use super::mapping::Mapping;

use super::{index::Indexing, tree::*};
/*use crate::{and, conjunction, connect, copy, every, every_n, not, or, pred, recycle, var};

pub fn skolemize<IDX: Indexing>(tree: &mut Tree<IDX>) {
    skolemize_recursive(tree, tree.output);
}

fn append_if_contain<IDX: Indexing>(tree: &Tree<IDX>, vars: &mut Vec<IDX>, idx: IDX, exclude: IDX) {
    let mut it = idx;
    let mut contrain = false;
    while it.is_addr() {
        let node = &tree[it];
        match node.symbol {
            Symbols::Variable { var_id } => {
                if var_id == exclude {
                    contrain = true;
                }
            }
            _ => panic!("Not a valid predicate"),
        }
        it = node.childs[0];
    }

    if contrain {
        let mut it = idx;
        while it.is_addr() {
            let node = &tree[it];
            match node.symbol {
                Symbols::Variable { var_id } => {
                    if var_id != exclude {
                        vars.push(var_id);
                    }
                }
                _ => panic!("Not a valid predicate"),
            }
            it = node.childs[0];
        }
    }
}

fn collect_variables<IDX: Indexing>(tree: &Tree<IDX>, vars: &mut Vec<IDX>, idx: IDX, exclude: IDX) {
    let node = tree[idx].clone();
    match node.symbol {
        Symbols::Predicate { .. } => {
            append_if_contain(tree, vars, node.childs[0], exclude);
        }
        _ => {
            for &child in node.childs() {
                if child.is_addr() {
                    collect_variables(tree, vars, child, exclude);
                }
            }
        }
    }
}

fn skolemize_recursive<IDX: Indexing>(tree: &mut Tree<IDX>, idx: IDX) {
    let node = tree[idx].clone();
    match node.symbol {
        Symbols::Exist { var_id } => {
            let mut vars: Vec<IDX> = Default::default();
            collect_variables(tree, &mut vars, node.childs[0], var_id);
            let tseitin = tree.add_anon();
            let skolem = tree.add_anon();
            let output = tree.output;
            tree.builder(conjunction!(
                connect!(output),
                every_n!(
                    &vars[..],
                    every!(
                        var_id,
                        or!(
                            pred!(id: tseitin,ids: &vars[..]),
                            not!(copy!(node.childs[0]))
                        )
                    )
                ),
                every_n!(
                    &vars[..],
                    or!(
                        pred!(id: tseitin, ids: &vars[..]),
                        pred!(id: skolem, ids: &vars[..])
                    )
                ),
                every_n!(
                    &vars[..],
                    every!(
                        var_id,
                        or!(pred!(id:skolem, ids:&vars[..]), not!(copy!(node.childs[0])))
                    )
                )
            ));
            tree.replace(recycle!(idx), pred!(id:tseitin,ids: &vars[..]));
        }
        _ => {}
    }
    for &child in node.childs() {
        if child.is_addr() {
            skolemize_recursive(tree, child);
        }
    }
}
*/
