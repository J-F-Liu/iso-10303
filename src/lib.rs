#![feature(assoc_char_funcs)]
#![feature(associated_type_bounds)]

pub mod express;
pub mod step;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
