use pom::char_class::{alpha, alphanum, digit, hex_digit};
use pom::parser::*;
use std::iter::FromIterator;
use std::str::{self, FromStr};

fn eol<'a>() -> Parser<'a, u8, u8> {
    (sym(b'\r') * sym(b'\n')) | sym(b'\n') | sym(b'\r')
}

fn remark<'a>() -> Parser<'a, u8, ()> {
    tail_remark() | embedded_remark()
}

fn tail_remark<'a>() -> Parser<'a, u8, ()> {
    seq(b"--") * none_of(b"\r\n").repeat(0..) * eol().discard()
}

fn embedded_remark<'a>() -> Parser<'a, u8, ()> {
    seq(b"(*")
        * (none_of(b"(*)").repeat(0..).discard() | embedded_remark()).repeat(0..)
        * seq(b"*)").discard()
}

fn identifier<'a>() -> Parser<'a, u8, &'a str> {
    let identifier = is_a(alpha) + (is_a(alphanum) | sym(b'_')).repeat(0..);
    identifier.collect().convert(str::from_utf8)
}

fn keyword<'a>(word: &'static str) -> Parser<'a, u8, ()> {
    identifier().convert(move |ident| {
        if ident.to_ascii_lowercase() == word {
            Ok(())
        } else {
            Err(())
        }
    })
}

fn boolean<'a>() -> Parser<'a, u8, bool> {
    keyword("flase").map(|_| false) | keyword("true").map(|_| true)
}

fn logical<'a>() -> Parser<'a, u8, Option<bool>> {
    keyword("flase").map(|_| Some(false))
        | keyword("true").map(|_| Some(true))
        | keyword("unknown").map(|_| None)
}

fn binary<'a, T: 'a + FromStr<Err: std::fmt::Debug>>() -> Parser<'a, u8, T> {
    let number = sym(b'%') * one_of(b"01").repeat(1..);
    number
        .convert(String::from_utf8)
        .convert(|s| T::from_str(&s))
}

fn integer<'a>() -> Parser<'a, u8, i64> {
    let number = one_of(b"+-").opt() + is_a(digit).repeat(1..);
    number
        .collect()
        .convert(str::from_utf8)
        .convert(|s| i64::from_str(s))
}

fn real<'a>() -> Parser<'a, u8, f64> {
    let integer = is_a(digit).repeat(1..);
    let frac = sym(b'.') + is_a(digit).repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + is_a(digit).repeat(1..);
    let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
    number
        .collect()
        .convert(str::from_utf8)
        .convert(f64::from_str)
}

fn string<'a>() -> Parser<'a, u8, String> {
    simple_string() | encoded_string()
}

fn simple_string<'a>() -> Parser<'a, u8, String> {
    let chars = sym(b'\'') * (none_of(b"'") | seq(b"''").map(|_| b'\'')).repeat(0..) - sym(b'\'');
    chars.convert(String::from_utf8)
}

// expect Unicode (ISO 10646) char codes
fn encoded_string<'a>() -> Parser<'a, u8, String> {
    let chars = sym(b'"')
        * (is_a(hex_digit).repeat(4).map(|digits| unsafe {
            char::from_u32(u32::from_str_radix(str::from_utf8_unchecked(&digits), 16).unwrap())
                .unwrap()
        }))
        .repeat(0..)
        - sym(b'"');
    chars.map(String::from_iter)
}
