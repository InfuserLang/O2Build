/*
 * Copyright (c) VisualDevelopment 2021-2021.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

use super::{typing::Type, Node};
use crate::abi::CallConv;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionPrototype {
    pub public: bool,
    pub call_conv: CallConv,
    pub symbol: String,
    pub args: Vec<Node>,
    pub ret_type: Type,
}
