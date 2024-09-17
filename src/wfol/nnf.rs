use super::tree::*;

pub fn to_nnf(tree: &mut Tree) {
    to_nnf_recursive(tree, tree.output)
}

fn to_nnf_recursive(tree: &mut Tree, idx: u32) {
    /*match *tree.get(idx) {
        Node::Not { inputs } => match *tree.get(inputs.0) {
            Node::Not { inputs } => todo!(),
            Node::And { inputs } => todo!(),
            Node::Or { inputs } => todo!(),
            Node::All { var_id, inputs } => todo!(),
            Node::Any { var_id, inputs } => todo!(),
            _ => {}
        },
        Node::And { inputs } => {
            to_nnf_recursive(tree, inputs.0);
            to_nnf_recursive(tree, inputs.1);
        }
        Node::Or { inputs } => {
            to_nnf_recursive(tree, inputs.0);
            to_nnf_recursive(tree, inputs.1);
        }
        Node::All { var_id, inputs } => {
            to_nnf_recursive(tree, inputs.0);
        }
        Node::Any { var_id, inputs } => {
            to_nnf_recursive(tree, inputs.0);
        }
        _ => {}
    }*/
}
