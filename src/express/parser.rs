use super::*;
use pom::char_class::{alpha, alphanum, digit, hex_digit};
use pom::parser::*;
use std::iter::FromIterator;
use std::ops::Range;
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

fn space<'a>() -> Parser<'a, u8, ()> {
    (one_of(b" \t\n\r\0").repeat(1..).discard() | remark())
        .repeat(0..)
        .discard()
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

fn simple_data_type<'a>() -> Parser<'a, u8, DataType> {
    keyword("number").map(|_| DataType::Number)
        | keyword("integer").map(|_| DataType::Integer)
        | keyword("boolean").map(|_| DataType::Boolean)
        | keyword("logical").map(|_| DataType::Logical)
        | (keyword("real") * (sym(b'(') * integer().map(|n| n as u8) - sym(b')')).opt())
            .map(|precision| DataType::Real { precision })
        | (keyword("string")
            * (sym(b'(') * integer().map(|n| n as usize) - sym(b')') - space()
                + keyword("fixed").opt())
            .opt())
        .map(|width_spec| {
            if let Some((width, fixed)) = width_spec {
                DataType::String {
                    width: Some(width),
                    fixed: fixed.is_some(),
                }
            } else {
                DataType::String {
                    width: None,
                    fixed: false,
                }
            }
        })
        | (keyword("binary")
            * (sym(b'(') * integer().map(|n| n as usize) - sym(b')') - space()
                + keyword("fixed").opt())
            .opt())
        .map(|width_spec| {
            if let Some((width, fixed)) = width_spec {
                DataType::Binary {
                    width: Some(width),
                    fixed: fixed.is_some(),
                }
            } else {
                DataType::Binary {
                    width: None,
                    fixed: false,
                }
            }
        })
}

fn bound_spec<'a>() -> Parser<'a, u8, Range<usize>> {
    (sym(b'[') * space() * integer().map(|n| n as usize) - space() - sym(b':') - space()
        + (integer().map(|n| n as usize) | sym(b'?').map(|_| usize::MAX))
        - space()
        - sym(b']'))
    .map(|(start, end)| start..end)
}

fn aggregation_data_type<'a>() -> Parser<'a, u8, DataType> {
    (keyword("array") * space() * sym(b'[') * space() * integer().map(|n| n as usize)
        - space()
        - sym(b':')
        - space()
        + integer().map(|n| n as usize)
        - space()
        - sym(b']')
        - space()
        - keyword("of")
        - space()
        + (keyword("optional") - space()).opt()
        + (keyword("unique") - space()).opt()
        + base_data_type())
    .map(
        |((((start, end), optional), unique), base_type)| DataType::Array {
            bound: start..end,
            optional: optional.is_some(),
            unique: unique.is_some(),
            base_type: Box::new(base_type),
        },
    ) | (keyword("list") * space() * bound_spec().opt() - space() - keyword("of") - space()
        + (keyword("unique") - space()).opt()
        + base_data_type())
    .map(|((bound, unique), base_type)| DataType::List {
        bound,
        unique: unique.is_some(),
        base_type: Box::new(base_type),
    }) | (keyword("bag") * space() * bound_spec().opt() - space() - keyword("of") - space()
        + base_data_type())
    .map(|(bound, base_type)| DataType::Bag {
        bound,
        base_type: Box::new(base_type),
    }) | (keyword("set") * space() * bound_spec().opt() - space() - keyword("of") - space()
        + base_data_type())
    .map(|(bound, base_type)| DataType::Set {
        bound,
        base_type: Box::new(base_type),
    })
}

fn type_ref<'a>() -> Parser<'a, u8, DataType> {
    identifier().map(|name| DataType::TypeRef {
        name: name.to_string(),
    })
}

fn base_data_type<'a>() -> Parser<'a, u8, DataType> {
    aggregation_data_type() | simple_data_type() | type_ref()
}

fn constructed_data_type<'a>() -> Parser<'a, u8, DataType> {
    (keyword("enumeration")
        * space()
        * keyword("of")
        * space()
        * sym(b'(')
        * list(identifier().map(str::to_string), sym(b',') - space())
        - sym(b')'))
    .map(|values| DataType::Enum { values })
        | (keyword("select")
            * space()
            * sym(b'(')
            * list(identifier().map(str::to_string), sym(b',') - space())
            - sym(b')'))
        .map(|types| DataType::Select { types })
}

fn underlying_type<'a>() -> Parser<'a, u8, DataType> {
    constructed_data_type() | aggregation_data_type() | simple_data_type() | type_ref()
}

fn named_type<'a>() -> Parser<'a, u8, Declaration> {
    (keyword("type") * space() * identifier() - space() - sym(b'=') - space() + underlying_type()
        - space()
        - sym(b';')
        + where_clause().opt()
        - keyword("end_type")
        - space()
        - sym(b';'))
    .map(
        |((type_id, underlying_type), domain_rules)| Declaration::Type {
            name: type_id.to_string(),
            underlying_type,
            domain_rules: domain_rules.unwrap_or(Vec::new()),
        },
    )
}

fn entity<'a>() -> Parser<'a, u8, Declaration> {
    let head = keyword("enity") * space() * identifier() - space()
        + supertype_declaration().opt()
        + subtype_declaration().opt()
        - sym(b';')
        - space();
    let body = attribute().repeat(0..)
        + derive_clause().opt()
        + inverse_clause().opt()
        + unique_clause().opt()
        + where_clause().opt();
    let tail = keyword("end_enity") - space() - sym(b';');
    (head + body - tail).map(
        |(
            ((entity_id, is_abstract), supertypes),
            ((((attributes, derived_attributes), _inverse_attributes), unique_rules), domain_rules),
        )| {
            Declaration::Entity {
                name: entity_id.to_string(),
                is_abstract: is_abstract == Some(true),
                supertypes: supertypes.unwrap_or(Vec::new()),
                attributes: attributes.into_iter().flatten().collect(),
                derives: derived_attributes.unwrap_or(Vec::new()),
                domain_rules: domain_rules.unwrap_or(Vec::new()),
                unique_rules: unique_rules.unwrap_or(Vec::new()),
            }
        },
    )
}

fn supertype_declaration<'a>() -> Parser<'a, u8, bool> {
    (keyword("abstract") * space() * keyword("supertype") * space() * subtype_constraint().opt())
        .map(|_| true)
        | keyword("supertype") * space() * subtype_constraint().map(|_| false)
}

fn subtype_constraint<'a>() -> Parser<'a, u8, ()> {
    keyword("of") * space() * sym(b'(') * supertype_expr() - sym(b')')
}

fn supertype_term<'a>() -> Parser<'a, u8, ()> {
    keyword("oneof")
        * space()
        * sym(b'(')
        * list(call(supertype_expr) - space(), sym(b',') - space()).discard()
        - sym(b')')
        | identifier().discard()
        | sym(b'(') * call(supertype_expr).discard() - sym(b')')
}
fn supertype_factor<'a>() -> Parser<'a, u8, ()> {
    list(supertype_term() - space(), keyword("and") - space()).discard()
}
fn supertype_expr<'a>() -> Parser<'a, u8, ()> {
    list(supertype_factor() - space(), keyword("andor") - space()).discard()
}

fn subtype_declaration<'a>() -> Parser<'a, u8, Vec<String>> {
    keyword("subtype")
        * space()
        * keyword("of")
        * space()
        * sym(b'(')
        * list(
            identifier().map(str::to_string) - space(),
            sym(b',') - space(),
        )
        - sym(b')')
}

fn attribute<'a>() -> Parser<'a, u8, Vec<Attribute>> {
    (list(
        identifier().map(str::to_string) - space(),
        sym(b',') - space(),
    ) - sym(b':')
        - space()
        + (keyword("optional") - space()).opt()
        + base_data_type()
        - space()
        - sym(b';'))
    .map(|((names, optional), data_type)| {
        names
            .into_iter()
            .map(|name| Attribute {
                name,
                data_type: data_type.clone(),
                optional: optional.is_some(),
            })
            .collect::<Vec<_>>()
    })
}

fn where_clause<'a>() -> Parser<'a, u8, Vec<DomainRule>> {
    let domain_rule = ((identifier().map(str::to_string) - space() - sym(b':') - space()).opt()
        + expression()
        - sym(b';')
        - space())
    .map(|(label, expr)| DomainRule { label, expr });
    keyword("where") * space() * domain_rule.repeat(1..)
}

fn derive_clause<'a>() -> Parser<'a, u8, Vec<DerivedAttribute>> {
    let derive_attribute = (identifier().map(str::to_string) - space() - sym(b':') - space()
        + base_data_type()
        - space()
        - sym(b'=')
        - space()
        + expression()
        - sym(b';')
        - space())
    .map(|((name, data_type), expr)| DerivedAttribute {
        name,
        data_type,
        expr,
    });
    keyword("derive") * space() * derive_attribute.repeat(1..)
}

fn inverse_clause<'a>() -> Parser<'a, u8, ()> {
    let inverse_attribute = identifier().map(str::to_string) - space() - sym(b':') - space()
        + ((keyword("set") | keyword("bag")) - space() + bound_spec().opt()
            - space()
            - keyword("of"))
        .opt()
        - space()
        + identifier()
        - space()
        - keyword("for")
        - space()
        + identifier()
        - space()
        - sym(b';')
        - space();
    keyword("inverse") * space() * inverse_attribute.repeat(1..).discard()
}

fn unique_clause<'a>() -> Parser<'a, u8, Vec<UniqueRule>> {
    let unique_rule = ((identifier().map(str::to_string) - space() - sym(b':') - space()).opt()
        + list(attribute_ref() - space(), sym(b',') - space()))
    .map(|(label, attributes)| UniqueRule { label, attributes });
    keyword("unique") * space() * unique_rule.repeat(1..)
}

fn attribute_ref<'a>() -> Parser<'a, u8, AttributeReference> {
    (keyword("self") * sym(b'\\') * identifier().map(str::to_string) - sym(b'.')
        + identifier().map(str::to_string))
    .map(|(entity, name)| AttributeReference {
        name,
        entity: Some(entity),
    }) | identifier().map(|name| AttributeReference {
        name: name.to_string(),
        entity: None,
    })
}

fn expression<'a>() -> Parser<'a, u8, Expression> {
    todo!()
}

fn declaration<'a>() -> Parser<'a, u8, Declaration> {
    named_type() | entity()
}

fn constants<'a>() -> Parser<'a, u8, Vec<Constant>> {
    let constant = (identifier().map(str::to_string) - space() - sym(b':') - space()
        + base_data_type()
        - space()
        - seq(b":=")
        - space()
        + expression()
        - sym(b';')
        - space())
    .map(|((name, data_type), expr)| Constant {
        name,
        data_type,
        expr,
    });
    keyword("constant") * space() * constant.repeat(1..)
        - keyword("end_constant")
        - space()
        - sym(b';')
        - space()
}

pub fn schema<'a>() -> Parser<'a, u8, Schema> {
    let head = space() * keyword("schema") * space() * identifier().map(str::to_string)
        - space()
        - sym(b':')
        - space();
    let body = constants().opt() + declaration().repeat(1..);
    let tail = keyword("end_schema") - space() - sym(b':') - space();
    (head + body - tail).map(|(name, (constants, declarations))| Schema {
        name,
        constants: constants.unwrap_or(Vec::new()),
        declarations,
    })
}
