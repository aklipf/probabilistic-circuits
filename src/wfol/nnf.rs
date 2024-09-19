use super::node::Symbols;
use super::{index::Indexing, tree::*};

pub fn to_nnf<IDX: Indexing>(tree: &mut Tree<IDX>) {
    to_nnf_recursive(tree, tree.output);
}

fn to_nnf_recursive<IDX: Indexing>(tree: &mut Tree<IDX>, idx: IDX) {
    let node = tree[idx].clone();
    match node.symbol {
        Symbols::Not => match tree[node.childs[0]].symbol {
            Symbols::Not => {
                let input = tree[node.childs[0]].childs[0];
                let output = tree.replace(
                    |recycler| recycler.cut(idx, &[input]),
                    |builder| builder.connect(input),
                );
                return to_nnf_recursive(tree, output);
            }
            Symbols::And => {
                let [left, right] = tree[node.childs[0]].childs;
                let output = tree.replace(
                    |recycler| recycler.cut(idx, &[left, right]),
                    |builder| {
                        builder.or(
                            |new_left| new_left.not(|not_left| not_left.connect(left)),
                            |new_right| new_right.not(|not_right| not_right.connect(right)),
                        )
                    },
                );
                return to_nnf_recursive(tree, output);
            }
            Symbols::Or => {
                let [left, right] = tree[node.childs[0]].childs;
                let output = tree.replace(
                    |recycler| recycler.cut(idx, &[left, right]),
                    |builder| {
                        builder.and(
                            |new_left| new_left.not(|not_left| not_left.connect(left)),
                            |new_right| new_right.not(|not_right| not_right.connect(right)),
                        )
                    },
                );
                return to_nnf_recursive(tree, output);
            }
            Symbols::All { var_id } => {
                let child = tree[node.childs[0]].childs[0];
                let output = tree.replace(
                    |recycler| recycler.cut(idx, &[child]),
                    |builder| builder.any(var_id, |any| any.not(|not| not.connect(child))),
                );
                return to_nnf_recursive(tree, output);
            }
            Symbols::Any { var_id } => {
                let child = tree[node.childs[0]].childs[0];
                let output = tree.replace(
                    |recycler| recycler.cut(idx, &[child]),
                    |builder| builder.all(var_id, |any| any.not(|not| not.connect(child))),
                );
                return to_nnf_recursive(tree, output);
            }
            _ => {}
        },
        _ => {}
    }
    for &child in node.childs() {
        if child.is_addr() {
            to_nnf_recursive(tree, child);
        }
    }
}
