use super::tree::*;

pub fn to_nnf(node: &Node) -> Node {
    match node {
        Node::Var(_) => node.clone(),
        Node::Weight(_) => todo!(),
        Node::Not(x) => match x.child.as_ref() {
            Node::Var(_) => node.clone(),
            Node::Weight(_) => todo!(),
            Node::Not(y) => to_nnf(&y.child),
            Node::And(y) => {
                let not_left: Node = not((*y.left).clone()).into();
                let not_right: Node = not((*y.right).clone()).into();
                or(to_nnf(&not_left), to_nnf(&not_right)).into()
            }
            Node::Or(y) => {
                let not_left: Node = not((*y.left).clone()).into();
                let not_right: Node = not((*y.right).clone()).into();
                and(to_nnf(&not_left), to_nnf(&not_right)).into()
            }
            Node::Imply(y) => {
                let not_right: Node = not((*y.right).clone()).into();
                and(to_nnf(&y.left), to_nnf(&not_right)).into()
            }
            Node::Equivalent(y) => {
                let not_left: Node = not((*y.left).clone()).into();
                let not_right: Node = not((*y.right).clone()).into();
                or(
                    and(to_nnf(&not_left), to_nnf(&y.right)),
                    and(to_nnf(&y.left), to_nnf(&not_right)),
                )
                .into()
            }
            Node::Predicate(_) => node.clone(),
            Node::Any(y) => {
                let a: Node = not((*y.expr).clone()).into();
                all(*y.var.clone(), to_nnf(&a)).into()
            }
            Node::All(y) => {
                let a: Node = not((*y.expr).clone()).into();
                any(*y.var.clone(), to_nnf(&a)).into()
            }
        },
        Node::And(x) => and(to_nnf(&x.left), to_nnf(&x.right)).into(),
        Node::Or(x) => or(to_nnf(&x.left), to_nnf(&x.right)).into(),
        Node::Imply(x) => {
            let not_left: Node = not((*x.left).clone()).into();
            or(to_nnf(&not_left), to_nnf(&x.right)).into()
        }
        Node::Equivalent(x) => {
            let not_left: Node = not((*x.left).clone()).into();
            let not_right: Node = not((*x.right).clone()).into();
            and(
                or(to_nnf(&not_left), to_nnf(&x.right)),
                or(to_nnf(&x.left), to_nnf(&not_right)),
            )
            .into()
        }
        Node::Predicate(_) => node.clone(),
        Node::Any(x) => any((*x.var).clone(), to_nnf(&x.expr)).into(),
        Node::All(x) => all((*x.var).clone(), to_nnf(&x.expr)).into(),
    }
}
