use std::{
    collections::{HashMap, HashSet},
    iter,
    sync::Mutex,
};

use itertools::Itertools;
use num_rational::Rational64;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Formula<T> {
    #[serde(rename = "leaf")]
    Leaf(T),
    #[serde(rename = "add")]
    Add(
        #[serde(rename = "lhs")] Box<Formula<T>>,
        #[serde(rename = "rhs")] Box<Formula<T>>,
    ),
    #[serde(rename = "sub")]
    Sub(
        #[serde(rename = "lhs")] Box<Formula<T>>,
        #[serde(rename = "rhs")] Box<Formula<T>>,
    ),
    #[serde(rename = "mul")]
    Mul(
        #[serde(rename = "lhs")] Box<Formula<T>>,
        #[serde(rename = "rhs")] Box<Formula<T>>,
    ),
    #[serde(rename = "div")]
    Div(
        #[serde(rename = "lhs")] Box<Formula<T>>,
        #[serde(rename = "rhs")] Box<Formula<T>>,
    ),
}

pub type IndexFormula = Formula<usize>;
pub type AssignedFormula = Formula<i64>;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[error("division by zero")]
pub struct ZeroDivisionError;

impl IndexFormula {
    fn apply(&self, values: &[i64]) -> Result<Rational64, ZeroDivisionError> {
        match self {
            Formula::Leaf(index) => Ok(Rational64::new(values[*index], 1)),
            Formula::Add(left, right) => Ok(left.apply(values)? + right.apply(values)?),
            Formula::Sub(left, right) => Ok(left.apply(values)? - right.apply(values)?),
            Formula::Mul(left, right) => Ok(left.apply(values)? * right.apply(values)?),
            Formula::Div(left, right) => {
                let left = left.apply(values)?;
                let right = right.apply(values)?;
                if right != Rational64::new(0, 1) {
                    Ok(left / right)
                } else {
                    Err(ZeroDivisionError)
                }
            }
        }
    }

    fn assign(&self, values: &[i64]) -> AssignedFormula {
        match self {
            Formula::Leaf(index) => Formula::Leaf(values[*index]),
            Formula::Add(left, right) => Formula::Add(
                Box::new(left.assign(values)),
                Box::new(right.assign(values)),
            ),
            Formula::Sub(left, right) => Formula::Sub(
                Box::new(left.assign(values)),
                Box::new(right.assign(values)),
            ),
            Formula::Mul(left, right) => Formula::Mul(
                Box::new(left.assign(values)),
                Box::new(right.assign(values)),
            ),
            Formula::Div(left, right) => Formula::Div(
                Box::new(left.assign(values)),
                Box::new(right.assign(values)),
            ),
        }
    }
}

impl AssignedFormula {
    pub fn format(&self) -> String {
        match self {
            Formula::Leaf(value) => value.to_string(),
            Formula::Add(left, right) => format!("({} + {})", left.format(), right.format()),
            Formula::Sub(left, right) => format!("({} - {})", left.format(), right.format()),
            Formula::Mul(left, right) => format!("({} * {})", left.format(), right.format()),
            Formula::Div(left, right) => format!("({} / {})", left.format(), right.format()),
        }
    }

    pub fn compute(&self) -> Result<Rational64, ZeroDivisionError> {
        match self {
            Formula::Leaf(value) => Ok(Rational64::new(*value, 1)),
            Formula::Add(left, right) => Ok(left.compute()? + right.compute()?),
            Formula::Sub(left, right) => Ok(left.compute()? - right.compute()?),
            Formula::Mul(left, right) => Ok(left.compute()? * right.compute()?),
            Formula::Div(left, right) => {
                let left = left.compute()?;
                let right = right.compute()?;
                if right != Rational64::new(0, 1) {
                    Ok(left / right)
                } else {
                    Err(ZeroDivisionError)
                }
            }
        }
    }
}

fn enumerate_formulas(indices: &[usize]) -> Vec<IndexFormula> {
    if indices.len() == 1 {
        return vec![Formula::Leaf(indices[0])];
    }

    let mut res = vec![];
    for break_at in 0..indices.len() {
        let left_indices = &indices[..break_at];
        let right_indices = &indices[break_at..];

        if left_indices.is_empty() || right_indices.is_empty() {
            continue;
        }

        for left_formula in enumerate_formulas(left_indices) {
            for right_formula in enumerate_formulas(right_indices) {
                res.push(Formula::Add(
                    Box::new(left_formula.clone()),
                    Box::new(right_formula.clone()),
                ));
                res.push(Formula::Sub(
                    Box::new(left_formula.clone()),
                    Box::new(right_formula.clone()),
                ));
                res.push(Formula::Mul(
                    Box::new(left_formula.clone()),
                    Box::new(right_formula.clone()),
                ));
                res.push(Formula::Div(
                    Box::new(left_formula.clone()),
                    Box::new(right_formula.clone()),
                ));
            }
        }
    }

    res
}

pub fn get_formulas_cached(n: usize) -> Vec<IndexFormula> {
    static FORMULAS_CACHE: Lazy<Mutex<HashMap<usize, Vec<IndexFormula>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    assert!(n <= 5, "too large n");

    {
        let cache = FORMULAS_CACHE.lock().unwrap();
        if let Some(formulas) = cache.get(&n) {
            // Clone cached formulas if already computed for given n
            return formulas.clone();
        }
    }

    // Compute and insert cache for this n
    let formulas = enumerate_formulas(&(0..n).collect_vec());

    {
        let mut cache = FORMULAS_CACHE.lock().unwrap();
        cache.insert(n, formulas.clone());
    }

    formulas
}

pub fn find_answers(cards: &[i64], target: i64) -> impl Iterator<Item = AssignedFormula> {
    // Clippy complains about unnecessary conversion to owned value, but it is indeed needed to
    // remove lifetime from the returning iterator.
    #[allow(clippy::unnecessary_to_owned)]
    let mut permutations = cards
        .to_vec()
        .into_iter()
        .permutations(cards.len())
        .unique();
    let formulas = get_formulas_cached(cards.len());

    let mut current_permutation = permutations.next();
    let mut current_formulas = formulas.clone().into_iter();

    iter::from_fn(move || loop {
        let Some(formula) = current_formulas.next() else {
            current_permutation = permutations.next();
            current_formulas = formulas.clone().into_iter();
            continue;
        };

        let Some(permutation) = current_permutation.clone() else {
            return None;
        };

        let Ok(value) = formula.apply(&permutation) else {
            continue;
        };

        if value == Rational64::from(target) {
            return Some(formula.assign(&permutation));
        }
    })
}

pub fn compute_impossibles(cards: &[i64], within: HashSet<i64>) -> HashSet<i64> {
    let mut impossibles = within;
    let formulas = get_formulas_cached(cards.len());

    'order_loop: for ordered in cards.iter().copied().permutations(cards.len()).unique() {
        for formula in &formulas {
            let Ok(value) = formula.apply(&ordered) else {
                continue;
            };

            if value.is_integer() {
                impossibles.remove(&value.to_integer());
            }

            if impossibles.is_empty() {
                break 'order_loop;
            }
        }
    }

    impossibles
}
