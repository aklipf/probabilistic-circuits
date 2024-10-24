use regex::Regex;
use std::fs;

use crate::{
    logic::propositional::{PMut, PropositionalTree},
    tree::{Addr, IndexedMutRef, Mapping},
};

pub fn load_file(file_name: String) -> Result<PropositionalTree, &'static str> {
    let contents = fs::read_to_string(file_name).expect("Not able to load file.");
    load_string(contents)
}

pub fn load_string(cnf: String) -> Result<PropositionalTree, &'static str> {
    let re_config =
        Regex::new(r"(?:^|\b)(?i:p)\s+(?i:cnf)\s+(?<vars>\d+)\s+(?<clauses>\d+)(?:$|\b)").unwrap();
    let re_remove_comments = Regex::new(r"(\p{L}\s.+)").unwrap();
    let re_clauses_vars = Regex::new(r"(?:^|[^-\w])(?<id>-?\d+)\b").unwrap();
    let re_clauses_sep = Regex::new(r"\b+0\b+").unwrap();

    // get CNF config (nb of variables and clauses)
    let Some(config) = re_config.captures(&cnf) else {
        return Err("Cannot find the problem line");
    };
    let n_clauses = config["clauses"].parse::<usize>().unwrap();
    let n_vars = config["vars"].parse::<usize>().unwrap();

    // remove all the comments
    let cleaned = re_remove_comments.replace_all(&cnf, "");

    // parse clauses

    let clauses: Vec<Vec<i32>> = re_clauses_sep
        .split(&cleaned)
        .filter_map(|clause| not_empty_ok(parse_clause(&re_clauses_vars, clause)))
        .collect();

    if clauses.len() != n_clauses {
        return Err("Inconsistent number of clauses");
    }

    let mut tree: PropositionalTree = Default::default();
    for _ in 0..n_vars {
        tree.add_anon();
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
fn add_var(builder: &mut IndexedMutRef<PropositionalTree>, var: i32) -> Addr {
    let var_addr = Addr::new((var.abs() - 1) as usize);
    if var > 0 {
        builder.var(var_addr)
    } else {
        builder.not(|inner| inner.var(var_addr))
    }
}

#[inline]
fn parse_clause(re: &Regex, clause: &str) -> Vec<i32> {
    re.captures_iter(clause)
        .map(|var| (var["id"].parse().unwrap()))
        .collect()
}

#[inline]
fn add_clause(builder: &mut IndexedMutRef<PropositionalTree>, vars: &[i32]) -> Addr {
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
fn add_clauses(builder: &mut IndexedMutRef<PropositionalTree>, clauses: &[Vec<i32>]) -> Addr {
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
    use crate::solver::naive::enumerate;

    use super::*;

    fn vec_to_u32(x: Vec<bool>) -> u32 {
        x.iter()
            .enumerate()
            .map(|(i, &v)| (if v { 1 } else { 0 }) << i)
            .sum()
    }

    #[test]
    fn test_cnf_parser() {
        let cnf = load_string(
            r#"
c Integration test of the CNF loader (unordered graph with 3 nodes and no self loops)
c by Astrid Klipfel
c
p cnf 9 9
-1 0
-5 0
-9 0
2 -4 0
-2 4 0
3 -7 0
-3 7 0
6 -8 0
-6 8 0
"#
            .to_string(),
        )
        .unwrap();

        assert_eq!(
            enumerate(&cnf).map(vec_to_u32).collect::<Vec<u32>>(),
            vec![
                0b000000000,
                0b000001010,
                0b001000100,
                0b001001110,
                0b010100000,
                0b010101010,
                0b011100100,
                0b011101110
            ]
        )
    }
}
