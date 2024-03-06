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
    Leaf(i32),
    Add(Box<Formula>, Box<Formula>),
    Sub(Box<Formula>, Box<Formula>),
    Mul(Box<Formula>, Box<Formula>),
    Div(Box<Formula>, Box<Formula>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ZeroDivision;

impl Formula {
    fn compute(&self) -> Result<Rational, ZeroDivision> {
        match self {
            Formula::Leaf(n) => Ok(Rational::new(*n, 1)),
            Formula::Add(left, right) => Ok(left.compute()? + right.compute()?),
            Formula::Sub(left, right) => Ok(left.compute()? - right.compute()?),
            Formula::Mul(left, right) => Ok(left.compute()? * right.compute()?),
            Formula::Div(left, right) => {
                let right = right.compute()?;
                if right != Rational::new(0, 1) {
                    Ok(left.compute()? / right)
                } else {
                    Err(ZeroDivision)
                }
            }
        }
    }
}

fn enumerate_formulas(cards: &[i32]) -> Vec<Formula> {
    if cards.len() == 1 {
        return vec![Formula::Leaf(cards[0])];
    }

    let mut res = vec![];
    for breakpoint in 0..cards.len() {
        let left = &cards[..breakpoint];
        let right = &cards[breakpoint..];

        if left.is_empty() || right.is_empty() {
            continue;
        }

        for left_formula in enumerate_formulas(left) {
            for right_formula in enumerate_formulas(right) {
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

fn compute_impossibles(cards: &[i32]) -> HashSet<i32> {
    let mut impossibles: HashSet<i32> = (1..=MAX_CARD_NUMBER).collect();

    'order_loop: for order in cards.iter().copied().permutations(5).unique() {
        for formula in enumerate_formulas(&order) {
            let Ok(value) = formula.compute() else {
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

fn main() {
    let entire = repeat_n(1..=MAX_CARD_NUMBER, CARD_DUPLICATES)
        .flatten()
        .sorted()
        .collect_vec();
    let combs = entire.iter().copied().combinations(NUM_HAND_CARDS).unique();

    for hand in combs {
        let impossibles = compute_impossibles(&hand);
        if !impossibles.is_empty() {
            println!("{hand:?} != {impossibles:?}");
        }
    }
}
