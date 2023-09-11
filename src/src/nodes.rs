#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{rc::Rc};

pub enum ALL_VARIANT {
    Number(Rc<NumberNode>),

    Add(Rc<AddNode>),
    Sub(Rc<SubtractNode>),
    Mul(Rc<MultiplyNode>),
    Div(Rc<DivideNode>),

    Plus(Rc<PlusNode>),
    Minus(Rc<MinusNode>),

    EmptyNode(),
}

/* ------------------------------------ */

pub struct NumberNode {
    pub value: f64,
}

impl NumberNode {
    fn __repr__(&self) -> String {
        return self.value.to_string()
    }

    pub fn new(value: f64) -> Self {
        return NumberNode { value: (value) }
    }
}

pub struct AddNode {
    pub node_a: ALL_VARIANT,
    pub node_b: ALL_VARIANT,
}

impl AddNode {
    fn __repr__(&self) -> String {
        return "".to_string()
    }

    pub fn new(node_a: ALL_VARIANT, node_b: ALL_VARIANT) -> Self {
        return AddNode { node_a: (node_a), node_b: (node_b) }
    }
}

pub struct SubtractNode {
    node_a: ALL_VARIANT,
    node_b: ALL_VARIANT,
}

impl SubtractNode {
    fn __repr__(&self) -> String {
        return "".to_string()
    }

    pub fn new(node_a: ALL_VARIANT, node_b: ALL_VARIANT) -> Self {
        return SubtractNode { node_a: (node_a), node_b: (node_b) }
    }
}

pub struct MultiplyNode {
    node_a: ALL_VARIANT,
    node_b: ALL_VARIANT,
}

impl MultiplyNode {
    fn __repr__(&self) -> String {
        return "".to_string()
    }

    pub fn new(node_a: ALL_VARIANT, node_b: ALL_VARIANT) -> Self {
        return MultiplyNode { node_a: (node_a), node_b: (node_b) }
    }
}

pub struct DivideNode {
    node_a: ALL_VARIANT,
    node_b: ALL_VARIANT,
}

impl DivideNode {
    fn __repr__(&self) -> String {
        return "".to_string()
    }

    pub fn new(node_a: ALL_VARIANT, node_b: ALL_VARIANT) -> Self {
        return DivideNode { node_a: (node_a), node_b: (node_b) }
    }
}

pub struct PlusNode {
    node: ALL_VARIANT,
}

impl PlusNode {
    fn __repr__(&self) -> String {
        return "".to_string()
    }

    pub fn new(node: ALL_VARIANT) -> Self {
        return PlusNode { node: (node) }
    }
}

pub struct MinusNode {
    node: ALL_VARIANT,
}

impl MinusNode {
    fn __repr__(&self) -> String {
        return "".to_string()
    }

    pub fn new(node: ALL_VARIANT) -> Self {
        return MinusNode { node: (node) }
    }
}
