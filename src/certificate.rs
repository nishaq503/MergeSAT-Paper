use std::collections::HashMap;

use crate::instance::Instance;

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

    pub fn empty() -> Certificate {
        Certificate {
            assignments: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn insert(&mut self, variable: u64, assignment: Assignment) -> () {
        self.assignments.insert(variable, assignment);
    }

    pub fn get(&self, literal: i64) -> Assignment {
        // TODO: try to make this less verbose
        match self.assignments.get(&(literal.abs() as u64)) {
            // variable is assigned a value in the certificate
            Some(&value) => match literal / literal.abs() {
                1 => value.clone(), // positive variable
                -1 => match value {
                    // negated variable
                    Assignment::True => Assignment::False,
                    Assignment::False => Assignment::True,
                    Assignment::Unassigned => panic!(
                        "Should not have gotten to third arm of inner match in certificate::get"
                    ),
                },
                _ => panic!("x / x.abs() should have been either 1 or -1"),
            },
            // variable is not assigned a value in the certificate
            None => Assignment::Unassigned,
        }
    }

    pub fn flatten(&self, instance: &Instance) -> Certificate {
        // TODO
        Certificate::new(HashMap::new())
    }
}
