#![allow(non_snake_case)]

use super::nodes::*;
use super::values::Number;

pub struct Interpreter {}

impl Interpreter {
    pub fn visit(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Number(_) => self.visit_NumberNode(node),
            ALL_VARIANT::Add(_) => self.visit_AddNode(node),
            ALL_VARIANT::Sub(_) => self.visit_SubtractNode(node),
            ALL_VARIANT::Mul(_) => self.visit_MultiplyNode(node),
            ALL_VARIANT::Div(_) => self.visit_DivideNode(node),
            _ => return Number { value: (0.0) },
        }
    }

    fn visit_NumberNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Number(inner) => {
                return Number {
                    value: (inner.value),
                }
            }
            _ => unreachable!(),
        };
    }

    fn visit_AddNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Add(_rc) => {
                let rc = &*_rc;
                return Number {
                    value: self.visit(rc.node_a.clone()).value
                        + self.visit(rc.node_b.clone()).value,
                };
            }

            _ => unreachable!(),
        }
    }

    fn visit_SubtractNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Sub(_rc) => {
                let rc = &*_rc;
                return Number {
                    value: self.visit(rc.node_a.clone()).value
                        - self.visit(rc.node_b.clone()).value,
                };
            }

            _ => unreachable!(),
        }
    }

    fn visit_MultiplyNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Mul(_rc) => {
                let rc = &*_rc;
                return Number {
                    value: self.visit(rc.node_a.clone()).value
                        * self.visit(rc.node_b.clone()).value,
                };
            }

            _ => unreachable!(),
        }
    }

    fn visit_DivideNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Div(_rc) => {
                let rc = &*_rc;
                return Number {
                    value: self.visit(rc.node_a.clone()).value
                        / self.visit(rc.node_b.clone()).value,
                };
            }

            _ => unreachable!(),
        }
    }
}
