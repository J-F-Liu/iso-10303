use iso_10303::express::parser::schema;
fn main() {
    let bytes = include_bytes!("../../schemas/example.exp");
    println!("{}", std::str::from_utf8(bytes).unwrap());
    let schema = schema().parse(bytes);
    println!("{:?}", schema);
}
