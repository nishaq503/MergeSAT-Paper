use std::fmt::{Display, Formatter, Result};

use crate::certificate::{Assignment, Certificate};
use crate::clause::{AppliedClause, Clause};

/// An Instance is represented as a Vec of Clauses.
/// For example, the vec [C_1, C_2, C_3] represents the instance (C_1 and C_2 and C_3).
/// An Instance has an associated vec of Certificates.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instance {
    clauses: Vec<Clause>,
    certificates: Vec<Certificate>,
    gimmes: Certificate,
}

/// Return type from application of a certificate to an instance.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AppliedInstance {
    True,
    False,
    Undecided(Instance),
}

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl Instance {
    pub fn new(clauses: Vec<Clause>) -> Instance {
        Instance {
            clauses,
            certificates: vec![],
            gimmes: Certificate::empty(),
        }
    }

    pub fn new_with_certificates(clauses: Vec<Clause>, certificates: Vec<Certificate>) -> Instance {
        Instance {
            clauses,
            certificates,
            gimmes: Certificate::empty(),
        }
    }

    pub fn new_from_other(other: &Instance) -> Instance {
        Instance {
            clauses: other.clauses.clone(),
            certificates: other.certificates.clone(),
            gimmes: other.gimmes.clone(),
        }
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

    pub fn apply(&self, certificate: &Certificate) -> AppliedInstance {
        let mut new_clauses: Vec<Clause> = vec![];
        for clause in self.clauses.iter() {
            let new_clause: AppliedClause = clause.apply(certificate);
            match new_clause {
                AppliedClause::True => {
                    continue;
                }
                AppliedClause::False => {
                    return AppliedInstance::False;
                }
                AppliedClause::Undecided(new_clause) => {
                    new_clauses.push(new_clause);
                }
            }
        }
        AppliedInstance::Undecided(Instance::new_with_certificates(
            self.clauses.clone(),
            self.certificates.clone(),
        ))
    }

    pub fn purify(&self) -> AppliedInstance {
        // TODO: Should this be a method in Certificate instead of Instance?
        let mut new_instance: Instance = Instance::new_from_other(self);
        let mut new_gimmes: Certificate = Certificate::empty();
        loop {
            for clause in new_instance.clauses.iter() {
                match clause.len() {
                    0 => {
                        return AppliedInstance::False;
                    }
                    1 => {
                        let literal = clause[0];
                        let assignment: Assignment = match literal / literal.abs() {
                            1 => Assignment::True,
                            -1 => Assignment::False,
                            _ => panic!("x / x.abs() should have been either 1 or -1"),
                        };
                        new_gimmes.insert(literal.abs() as u64, assignment);
                        new_instance
                            .gimmes
                            .insert(literal.abs() as u64, assignment);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            if new_gimmes.len() == 0 {
                break;
            }
            else {
                match new_instance.apply(&new_gimmes) {
                    AppliedInstance::True => {
                        return AppliedInstance::True;
                    }
                    AppliedInstance::False => {
                        return AppliedInstance::False;
                    }
                    AppliedInstance::Undecided(instance) => {
                        new_instance = instance;
                    }
                }
            }
        }
        AppliedInstance::Undecided(new_instance)
    }
}
