use std::str::Chars;

use itertools::Itertools;

use crate::{
    incr_pos_by,
    token::{Token, TokenPos},
};

#[inline]
pub(crate) fn tokenise_digit(pos: &mut TokenPos, c: char, it: &mut Chars) -> Token {
    let lit = c.to_string() + &it.take_while_ref(char::is_ascii_digit).collect::<String>();
    Token::Integer(incr_pos_by!(pos, lit.len() - 1), lit)
}

#[inline]
pub(crate) fn tokenise_str(pos: &mut TokenPos, it: &mut Chars) -> Token {
    let lit = it.take_while_ref(|c| *c != '"').collect::<String>();
    assert_eq!(it.next(), Some('"'));
    Token::String(incr_pos_by!(pos, lit.len() + 1), lit)
}

#[inline]
pub(crate) fn tokenise_char(pos: &mut TokenPos, it: &mut Chars) -> Token {
    let c = it.next().unwrap();
    assert_ne!(c, '\'');
    assert_eq!(it.next(), Some('\''));
    Token::Char(incr_pos_by!(pos, 2), c)
}