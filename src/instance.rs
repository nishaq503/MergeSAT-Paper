use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;

/// A Clause is represented as a Vec of integers.
/// Each integer value represents a variable, and the sign represents whether a variable is negated.
/// For example, the vec [3, -1, 4] represents the clause (x_3 or (not x_1) or x_4).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Clause {
    literals: Vec<i64>,
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

    pub fn apply(&self, certificate: &Certificate) -> Clause {
        // TODO: Redo after proper API design
        let mut literals: Vec<i64> = vec![];
        for &literal in self.literals.iter() {
            match certificate.get(literal) {
                Assignment::False => { continue; },
                _ => literals.push(literal)
            }
        }
        Clause::new(literals)
    }
}

/// An Instance is represented as a Vec of Clauses.
/// For example, the vec [C_1, C_2, C_3] represents the instance (C_1 and C_2 and C_3).
/// An Instance has an associated vec of Certificates.
#[derive(Debug, PartialEq, Eq)]
pub struct Instance {
    clauses: Vec<Clause>,
    certificates: Vec<Certificate>,
}

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Instance {
    pub fn new(clauses: Vec<Clause>, certificates: Vec<Certificate>) -> Instance {
        Instance { clauses, certificates }
    }

    pub fn to_string(&self) -> String {
        self.clauses
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn len(&self) -> usize {
        self.clauses.len()
    }

    pub fn apply(&self, certificate: &Certificate) -> Instance {
        // TODO
        Instance::new(self.clauses.clone(), self.certificates.clone())
    }
}

/// An Assignment for a variable is either True, False, or Unassigned.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Assignment {
    True = 1,
    False = -1,
    Unassigned = 0,
}

/// A certificate is represented by a HashMap of Index to Assignment,
/// where the index represents a variable.
/// A certificate knows which Instance it is associated with.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Certificate {
    assignments: HashMap<u64, Assignment>,
}

impl Certificate {
    pub fn new(assignments: HashMap<u64, Assignment>) -> Certificate {
        Certificate { assignments }
    }

    pub fn get(&self, literal: i64) -> Assignment {
        // TODO: make this less verbose
        match self.assignments.get(&(literal.abs() as u64)) {
            Some(&value) => {
                match literal.signum() {
                    1 => value.clone(),
                    _ => {
                        match value {
                            Assignment::True => Assignment::False,
                            Assignment::False => Assignment::True,
                            Assignment::Unassigned => Assignment::Unassigned,
                        }
                    }
                }
            }
            None => Assignment::Unassigned
        }
    }

    pub fn flatten(&self, instance: &Instance) -> Certificate {
        // TODO
        Certificate::new(HashMap::new())
    }
}
