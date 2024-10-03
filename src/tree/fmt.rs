/*impl<IDX: Indexing> Node<IDX> {
    fn fmt_name(tree: &Tree<IDX>, id: IDX) -> String {
        if let Some(vname) = tree.get_named(id) {
            vname.to_owned()
        } else {
            format!("Anon{}", id.addr())
        }
    }

    pub(super) fn fmt_recursive(
        &self,
        tree: &Tree<IDX>,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self.symbol() {
            Symbols::Variable { var_id } => {
                write!(f, "{}", Self::fmt_name(tree, *var_id))
            }
            Symbols::Not => {
                write!(f, "\u{00AC}")?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)
            }
            Symbols::And => {
                write!(f, "(")?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, "\u{2227}")?;
                tree.nodes[self.childs()[1].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::Or => {
                write!(f, "(")?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, "\u{2228}")?;
                tree.nodes[self.childs()[1].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::Predicate { pred_id } => {
                write!(f, "{}(", Self::fmt_name(tree, *pred_id))?;

                if self.num_childs() == 0 {
                    write!(f, ")")
                } else {
                    let mut node = &tree.nodes[self.childs()[0].addr()];
                    node.fmt_recursive(tree, f)?;
                    while node.num_childs() != 0 {
                        node = &tree.nodes[node.childs()[0].addr()];
                        write!(f, ", ")?;
                        node.fmt_recursive(tree, f)?;
                    }
                    write!(f, ")")
                }
            }
            Symbols::Every { var_id } => {
                write!(f, "\u{2200}{}:(", Self::fmt_name(tree, *var_id))?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::Exist { var_id } => {
                write!(f, "\u{2203}{}:(", Self::fmt_name(tree, *var_id))?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::None => panic!("Unkown node None"),
        }
    }
}*/

#[cfg(test)]
mod tests {

    #[test]
    fn test_fmt() {
        todo!()
        /*
        let tree: Tree = Tree::build(
            every!(name:"x", exist!(name:"y", and!(or!(not!(var!(name:"A")), var!(name:"x")), var!(name:"y")))),
        );
        assert_eq!(format!("{tree}"), "∀x:(∃y:(((¬A∨x)∧y)))");
        let tree: Tree = Tree::build(every!(
            name:"x",
            exist!(
                name:"y",
                and!(
                    pred!(name:"pred_x", name:"x"),
                    pred!(name:"pred_xy", name:"x", name:"y")
                )
            )
        ));
        assert_eq!(format!("{tree}"), "∀x:(∃y:((pred_x(x)∧pred_xy(x, y))))"); */
    }
}
