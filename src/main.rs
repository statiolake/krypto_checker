use std::{
    collections::HashSet,
    ops::{Add, Div, Mul, Sub},
};

use itertools::{repeat_n, Itertools};

const MAX_CARD_NUMBER: i32 = 10;
const CARD_DUPLICATES: usize = 3;
const NUM_HAND_CARDS: usize = 5;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Rational {
    num: i32,
    den: i32,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Self {
        assert_ne!(den, 0);
        Rational { num, den }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Rational::new(self.num * other.den, self.den * other.num)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num * other.den == other.num * self.den
    }
}

impl TryFrom<Rational> for i32 {
    type Error = Rational;

    fn try_from(value: Rational) -> Result<Self, Self::Error> {
        if value.num % value.den == 0 {
            Ok(value.num / value.den)
        } else {
            Err(value)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Formula {
    Leaf(usize),
    Add(Box<Formula>, Box<Formula>),
    Sub(Box<Formula>, Box<Formula>),
    Mul(Box<Formula>, Box<Formula>),
    Div(Box<Formula>, Box<Formula>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ZeroDivisionError;

impl Formula {
    fn apply(&self, values: &[i32]) -> Result<Rational, ZeroDivisionError> {
        match self {
            Formula::Leaf(index) => Ok(Rational::new(values[*index], 1)),
            Formula::Add(left, right) => Ok(left.apply(values)? + right.apply(values)?),
            Formula::Sub(left, right) => Ok(left.apply(values)? - right.apply(values)?),
            Formula::Mul(left, right) => Ok(left.apply(values)? * right.apply(values)?),
            Formula::Div(left, right) => {
                let left = left.apply(values)?;
                let right = right.apply(values)?;
                if right != Rational::new(0, 1) {
                    Ok(left / right)
                } else {
                    Err(ZeroDivisionError)
                }
            }
        }
    }
}

fn enumerate_formulas(indices: &[usize]) -> Vec<Formula> {
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

fn compute_impossibles(formulas: &[Formula], cards: &[i32]) -> HashSet<i32> {
    let mut impossibles: HashSet<i32> = (1..=MAX_CARD_NUMBER).collect();

    'order_loop: for order in permutations(cards).into_iter().unique() {
        for formula in formulas {
            let Ok(value) = formula.apply(&order) else {
                continue;
            };

            if let Ok(integer) = i32::try_from(value) {
                impossibles.remove(&integer);
            }

            if impossibles.is_empty() {
                break 'order_loop;
            }
        }
    }

    impossibles
}

fn combinations(values: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }

    if values.len() < k {
        return vec![];
    }

    let mut combs = vec![];

    for i in 0..values.len() {
        for rest in combinations(&values[i + 1..], k - 1) {
            let mut hand = vec![values[i]];
            hand.extend(rest);
            combs.push(hand.clone());
        }
    }

    combs
}

fn permutations(values: &[i32]) -> Vec<Vec<i32>> {
    if values.len() <= 1 {
        return vec![values.to_vec()];
    }

    let mut perms = vec![];

    for i in 0..values.len() {
        let mut values = values.to_vec();
        let picked = values.remove(i);
        for rest in permutations(&values) {
            let mut hand = vec![picked];
            hand.extend(rest);
            perms.push(hand.clone());
        }
    }

    perms
}

fn main() {
    let entire = repeat_n(1..=MAX_CARD_NUMBER, CARD_DUPLICATES)
        .flatten()
        .sorted()
        .collect_vec();
    let combs = combinations(&entire, NUM_HAND_CARDS).into_iter().unique();
    let combs = combs.into_iter().unique();
    let formulas = enumerate_formulas(&(0..NUM_HAND_CARDS).collect_vec());

    for hand in combs {
        let impossibles = compute_impossibles(&formulas, &hand);
        if !impossibles.is_empty() {
            println!("{hand:?} != {impossibles:?}");
        }
    }
}
