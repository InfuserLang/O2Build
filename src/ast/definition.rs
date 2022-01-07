use std::slice::Iter;

use cranelift::prelude::isa::CallConv;
use debug_tree::add_branch;

use super::function::{parse_callconv, parse_func_def};
use crate::{
    ast::Node,
    match_token, next_token,
    token::{Keyword, Token},
};

pub(crate) fn parse_def(
    public: bool,
    external: bool,
    it: &mut Iter<Token>,
) -> Result<Node, Option<Token>> {
    add_branch!("parse_def");
    let token = next_token!(it, return Err(None));

    match token {
        Token::Keyword(_, Keyword::Function) => {
            parse_func_def(public, external, CallConv::SystemV, it)
        }
        Token::Keyword(_, Keyword::CallConv) => {
            let abi = parse_callconv(it)?;
            match_token!(it.next(), Token::Keyword(_, Keyword::Function), Ok(()))?;

            parse_func_def(public, external, abi, it)
        }
        _ => Err(Some(token.clone())),
    }
}
