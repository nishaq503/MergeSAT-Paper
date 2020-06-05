use std::fmt::{Display, Formatter, Result};

use crate::certificate::{Assignment, Certificate};
use std::ops::Index;

/// A Clause is represented as a Vec of integers.
/// Each integer value represents a variable, and the sign represents whether a variable is negated.
/// For example, the vec [3, -1, 4] represents the clause (x_3 or (not x_1) or x_4).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Clause {
    literals: Vec<i64>,
}

/// Return type from application of a certificate to a clause.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AppliedClause {
    True,
    False,
    Undecided(Clause),
}

impl Index<usize> for Clause {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.literals[index]
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Clause {
    pub fn new(literals: Vec<i64>) -> Clause {
        Clause { literals }
    }

    pub fn to_string(&self) -> String {
        self.literals
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn contains(&self, variable: u64) -> bool {
        if self
            .literals
            .iter()
            .any(|&literal| (literal.abs() as u64) == variable)
        {
            true
        } else {
            false
        }
    }

    pub fn apply(&self, certificate: &Certificate) -> AppliedClause {
        let mut new_literals: Vec<i64> = vec![];
        for &literal in self.literals.iter() {
            match certificate.get(literal) {
                Assignment::True => {
                    return AppliedClause::True;
                }
                Assignment::False => {
                    continue;
                }
                Assignment::Unassigned => {
                    new_literals.push(literal);
                }
            }
        }
        AppliedClause::Undecided(Clause::new(new_literals))
    }
}
