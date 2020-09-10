use super::*;
use pom::char_class::{alpha, alphanum, digit, hex_digit};
use pom::parser::*;
use std::str::{self, FromStr};

fn comment<'a>() -> Parser<'a, u8, ()> {
    seq(b"/*") * (!seq(b"*/") * any()).repeat(0..) * seq(b"*/").discard()
}

fn space<'a>() -> Parser<'a, u8, ()> {
    (one_of(b" \t\n\r\0").repeat(1..).discard() | comment())
        .repeat(0..)
        .discard()
}

fn identifier<'a>() -> Parser<'a, u8, &'a str> {
    let identifier = is_a(alpha) + (is_a(alphanum) | sym(b'_')).repeat(0..);
    identifier.collect().convert(str::from_utf8)
}

fn integer<'a>() -> Parser<'a, u8, i64> {
    let number = one_of(b"+-").opt() + is_a(digit).repeat(1..);
    number.collect().convert(str::from_utf8).convert(|s| i64::from_str(s))
}

fn real<'a>() -> Parser<'a, u8, f64> {
    let integer = is_a(digit).repeat(1..);
    let frac = sym(b'.') + is_a(digit).repeat(1..);
    let exp = one_of(b"eE") + one_of(b"+-").opt() + is_a(digit).repeat(1..);
    let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
    number.collect().convert(str::from_utf8).convert(f64::from_str)
}

fn string<'a>() -> Parser<'a, u8, String> {
    let special_char = sym(b'\\')
        | sym(b'b').map(|_| b'\x08')
        | sym(b'f').map(|_| b'\x0C')
        | sym(b'n').map(|_| b'\n')
        | sym(b'r').map(|_| b'\r')
        | sym(b't').map(|_| b'\t');
    let escape_sequence = sym(b'\\') * special_char;
    let chars = sym(b'\'') * (none_of(b"\\'") | escape_sequence).repeat(0..) - sym(b'\'');
    chars.convert(String::from_utf8)
}

fn entity_id<'a>() -> Parser<'a, u8, i64> {
    sym(b'#') * integer()
}

fn constant_ref<'a>() -> Parser<'a, u8, String> {
    sym(b'#') * identifier().map(str::to_string)
}

// fn value_name<'a>() -> Parser<'a, u8, String> {
//     sym(b'@') * identifier().map(str::to_string)
// }

// fn resource<'a>() -> Parser<'a, u8, String> {
//     sym(b'<') * identifier().map(str::to_string) - sym(b'>')
// }

fn binary<'a>() -> Parser<'a, u8, ()> {
    // todo: decode binary value
    seq(b"\"\"") * one_of(b"0123") * is_a(hex_digit).repeat(0..).discard() - seq(b"\"\"")
}

fn enum_value<'a>() -> Parser<'a, u8, String> {
    sym(b'.') * identifier().map(str::to_string) - sym(b'.')
}

fn parameter<'a>() -> Parser<'a, u8, Parameter> {
    typed_parameter().map(|param| Parameter::TypedParameter(param))
        | untyped_parameter().map(|param| Parameter::UnTypedParameter(param))
        | sym(b'*').map(|_| Parameter::OmittedParameter)
}

fn parameter_list<'a>() -> Parser<'a, u8, Vec<Parameter>> {
    sym(b'(') * space() * list(call(parameter) - space(), sym(b',') - space()) - sym(b')')
}

fn typed_parameter<'a>() -> Parser<'a, u8, TypedParameter> {
    (identifier().map(str::to_string) - space() + parameter_list())
        .map(|(type_name, parameters)| TypedParameter { type_name, parameters })
}

fn untyped_parameter<'a>() -> Parser<'a, u8, UnTypedParameter> {
    parameter_list().map(|parameters| UnTypedParameter::List(parameters))
        | enum_value().map(|value| UnTypedParameter::EnumValue(value))
        | entity_id().map(|id| UnTypedParameter::EntityRef(id))
        | constant_ref().map(|name| UnTypedParameter::ConstantRef(name))
        | real().map(|value| UnTypedParameter::Real(value))
        | integer().map(|value| UnTypedParameter::Integer(value))
        | string().map(|value| UnTypedParameter::String(value))
        | binary().map(|value| UnTypedParameter::Binary(value))
        | sym(b'$').map(|_| UnTypedParameter::Null)
}

fn entity_instance<'a>() -> Parser<'a, u8, EntityInstance> {
    (entity_id() - space() - sym(b'=') - space()
        + (typed_parameter().map(|value| vec![value])
            | sym(b'(') * space() * typed_parameter().repeat(1..) - space() - sym(b')'))
        - space()
        - sym(b';')
        - space())
    .map(|(id, value)| EntityInstance { id, value })
}

pub fn exchange_file<'a>() -> Parser<'a, u8, ExchangeFile> {
    let header_entity = typed_parameter() - space() - sym(b';') - space();
    let head = seq(b"ISO-10303-21;") * space() * seq(b"HEADER;") * space() * header_entity.repeat(3..)
        - seq(b"ENDSEC;")
        - space();
    let data = seq(b"DATA;") * space() * entity_instance().repeat(0..) - seq(b"ENDSEC;") - space();
    let tail = seq(b"END-ISO-10303-21;") - space();
    (head + data - tail).map(|(header, data)| ExchangeFile { header, data })
}
