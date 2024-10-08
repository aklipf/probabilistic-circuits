/*use regex::Regex;
use std::fs;

use crate::tree::allocator::Allocator;
use crate::tree::builder::Builder;
use crate::tree::index::Indexing;
use crate::tree::mapping::Mapping;
use crate::tree::tree::Tree;

pub fn load_file<IDX: Indexing>(file_name: String) -> Result<Tree<IDX>, &'static str> {
    let contents = fs::read_to_string(file_name).expect("Not able to load file.");
    load_string(contents)
}

pub fn load_string<IDX: Indexing>(cnf: String) -> Result<Tree<IDX>, &'static str> {
    let re_config =
        Regex::new(r"(?:^|\b)(?i:p)\s+(?i:cnf)\s+(?<vars>\d+)\s+(?<clauses>\d+)(?:$|\b)").unwrap();
    let re_remove_comments = Regex::new(r"(\p{L}\s.+)").unwrap();
    let re_clauses_vars = Regex::new(r"(?:^|[^-\w])(?<not>-)?(?<id>\d+)\b").unwrap();
    let re_clauses_sep = Regex::new(r"\b+0\b+").unwrap();

    // get CNF config (nb of variables and clauses)
    let Some(config) = re_config.captures(&cnf) else {
        return Err("Cannot find the problem line");
    };
    let n_vars = config["vars"].parse::<usize>().unwrap();
    let n_clauses = config["clauses"].parse::<usize>().unwrap();

    // remove all the comments
    let cleaned = re_remove_comments.replace_all(&cnf, "");

    // parse clauses
    let mut tree: Tree<IDX> = Default::default();

    let clauses: Vec<Vec<(bool, IDX)>> = re_clauses_sep
        .split(&cleaned)
        .filter_map(|clause| not_empty_ok(parse_clause(&re_clauses_vars, &mut tree, clause)))
        .collect();

    if clauses.len() != n_clauses {
        return Err("Inconsistent number of clauses");
    }

    tree.builder(|builder| add_clauses(builder, &clauses));

    Ok(tree)
}

#[inline]
fn not_empty_ok<T>(vec: Vec<T>) -> Option<Vec<T>> {
    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

#[inline]
fn add_var<IDX: Indexing, P: Allocator<IDX = IDX> + Mapping<IDX>>(
    builder: &mut Builder<'_, IDX, P>,
    var: (bool, IDX),
) -> IDX {
    let (not, var_id) = var;
    if not {
        builder.not(|inner| inner.var(var_id))
    } else {
        builder.var(var_id)
    }
}

#[inline]
fn parse_clause<IDX: Indexing, M: Mapping<IDX>>(
    re: &Regex,
    map: &mut M,
    clause: &str,
) -> Vec<(bool, IDX)> {
    re.captures_iter(clause)
        .map(|var| {
            (
                var.name("not").is_some(),
                map.add_named(&format!("x{}", &var["id"]).to_string()),
            )
        })
        .collect()
}

#[inline]
fn add_clause<IDX: Indexing, P: Allocator<IDX = IDX> + Mapping<IDX>>(
    builder: &mut Builder<'_, IDX, P>,
    vars: &[(bool, IDX)],
) -> IDX {
    if vars.len() == 1 {
        add_var(builder, vars[0])
    } else {
        builder.or(
            |left| add_var(left, vars[0]),
            |right| add_clause(right, &vars[1..]),
        )
    }
}

#[inline]
fn add_clauses<IDX: Indexing, P: Allocator<IDX = IDX> + Mapping<IDX>>(
    builder: &mut Builder<'_, IDX, P>,
    clauses: &[Vec<(bool, IDX)>],
) -> IDX {
    if clauses.len() == 1 {
        add_clause(builder, &clauses[0])
    } else {
        builder.and(
            |left| add_clause(left, &clauses[0]),
            |right| add_clauses(right, &clauses[1..]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_cnf_parser() {
        let cnt = load_string::<u32>(
            r#"
c Integration test of the CNF loader
c by Astrid Klipfel
c
p cnf 5 4
2 3 -5 0
-1 3 -4 0
1 -3 4 5 0
1 -2 0
"#
            .to_string(),
        )
        .unwrap();

        let expected = Tree::build(expr!(
            (x2 | x3 | !x5) & (!x1 | x3 | !x4) & (x1 | !x3 | x4 | x5) & (x1 | !x2)
        ));

        assert_eq!(format!("{cnt}"), format!("{expected}"));
    }
}
*/
