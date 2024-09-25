use crate::{and, connect, every, exist, not, or, recycle};

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
                let output = tree.replace(recycle!(idx, input), connect!(input));
                return to_nnf_recursive(tree, output);
            }
            Symbols::And => {
                let [left, right] = tree[node.childs[0]].childs;
                let output = tree.replace(
                    recycle!(idx, left, right),
                    or!(not!(connect!(left)), not!(connect!(right))),
                );
                return to_nnf_recursive(tree, output);
            }
            Symbols::Or => {
                let [left, right] = tree[node.childs[0]].childs;
                let output = tree.replace(
                    recycle!(idx, left, right),
                    and!(not!(connect!(left)), not!(connect!(right))),
                );
                return to_nnf_recursive(tree, output);
            }
            Symbols::Every { var_id } => {
                let child = tree[node.childs[0]].childs[0];
                let output =
                    tree.replace(recycle!(idx, child), exist!(var_id, not!(connect!(child))));
                return to_nnf_recursive(tree, output);
            }
            Symbols::Exist { var_id } => {
                let child = tree[node.childs[0]].childs[0];
                let output =
                    tree.replace(recycle!(idx, child), every!(var_id, not!(connect!(child))));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_to_nnf() {
        let mut tree = Tree::build(not!(
            every!(name:"x",exist!(name:"y",imply!(or!(not!(and!(not!(var!(name:"A")),var!(name:"B"))),var!(name:"C")),pred!(name:"test",name:"x",name:"y"))))
        ));
        to_nnf(&mut tree);

        assert_eq!(format!("{tree}"), "∃x:(∀y:((((A∨¬B)∨C)∧¬test(x, y))))");
    }
}
