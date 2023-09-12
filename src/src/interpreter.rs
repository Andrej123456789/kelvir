#![allow(non_snake_case)]

use super::nodes::*;
use super::values::{Number};

pub struct Interpreter {

}

macro_rules! perform_operation {
    ($inner:expr, $operator:expr) => {{
        let mut value_a = 1.0;
        let mut value_b = 1.0;

        match $inner.node_a.clone() {
            ALL_VARIANT::Number(rc) => {
                let number = &*rc;
                value_a = number.value;
            }
            _ => unreachable!(),
        }

        match $inner.node_b.clone() {
            ALL_VARIANT::Number(rc) => {
                let number = &*rc;
                value_b = number.value;
            }
            _ => unreachable!(),
        }

        Number { value: $operator(value_a, value_b) }
    }};
}

impl Interpreter {
    pub fn visit(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Number(_) => self.visit_NumberNode(node),
            ALL_VARIANT::Add(_) => self.visit_AddNode(node),
            ALL_VARIANT::Sub(_) => self.visit_SubtractNode(node),
            ALL_VARIANT::Mul(_) => self.visit_MultiplyNode(node),
            ALL_VARIANT::Div(_) => self.visit_DivideNode(node),
            _ => return Number { value: (0.0) }
        }
    }

    fn visit_NumberNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Number(inner) => return Number {value: (inner.value) },
            _ => unreachable!(),
        };
    }

    fn visit_AddNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Add(inner) => return perform_operation!(inner, |a, b| a + b),
            ALL_VARIANT::Sub(inner) => return perform_operation!(inner, |a, b| a + b),
            ALL_VARIANT::Mul(inner) => return perform_operation!(inner, |a, b| a + b),
            ALL_VARIANT::Div(inner) => return perform_operation!(inner, |a, b| a + b),
            _ => unreachable!(),
        };
    }

    fn visit_SubtractNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Add(inner) => return perform_operation!(inner, |a, b| a - b),
            ALL_VARIANT::Sub(inner) => return perform_operation!(inner, |a, b| a - b),
            ALL_VARIANT::Mul(inner) => return perform_operation!(inner, |a, b| a - b),
            ALL_VARIANT::Div(inner) => return perform_operation!(inner, |a, b| a - b),
            _ => unreachable!(),
        };
    }

    fn visit_MultiplyNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Add(inner) => return perform_operation!(inner, |a, b| a * b),
            ALL_VARIANT::Sub(inner) => return perform_operation!(inner, |a, b| a * b),
            ALL_VARIANT::Mul(inner) => return perform_operation!(inner, |a, b| a * b),
            ALL_VARIANT::Div(inner) => return perform_operation!(inner, |a, b| a * b),
            _ => unreachable!(),
        };
    }

    fn visit_DivideNode(&self, node: ALL_VARIANT) -> Number {
        match node {
            ALL_VARIANT::Add(inner) => return perform_operation!(inner, |a, b| a / b),
            ALL_VARIANT::Sub(inner) => return perform_operation!(inner, |a, b| a / b),
            ALL_VARIANT::Mul(inner) => return perform_operation!(inner, |a, b| a / b),
            ALL_VARIANT::Div(inner) => return perform_operation!(inner, |a, b| a / b),
            _ => unreachable!(),
        };
    }
}
