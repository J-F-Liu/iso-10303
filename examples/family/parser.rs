pub struct Unimplemented {}
type Date = [i64; 3usize];
pub enum HairType {
    Blonde,
    Brown,
    Black,
    Red,
    White,
}
pub trait IPerson {
    fn first_name(&self) -> &String;
    fn last_name(&self) -> &String;
    fn nickname(&self) -> &String;
    fn birth_date(&self) -> &Date;
    fn children(&self) -> &std::collections::HashSet<Box<dyn IPerson>>;
    fn hair(&self) -> &HairType;
}
pub trait IFemale: IPerson {}
pub struct Female {
    first_name: String,
    last_name: String,
    nickname: String,
    birth_date: Date,
    children: std::collections::HashSet<Box<dyn IPerson>>,
    hair: HairType,
}
impl IPerson for Female {
    fn first_name(&self) -> &String {
        &self.first_name
    }
    fn last_name(&self) -> &String {
        &self.last_name
    }
    fn nickname(&self) -> &String {
        &self.nickname
    }
    fn birth_date(&self) -> &Date {
        &self.birth_date
    }
    fn children(&self) -> &std::collections::HashSet<Box<dyn IPerson>> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl IFemale for Female {}
pub trait IMale: IPerson {
    fn wife(&self) -> &Box<dyn IFemale>;
}
pub struct Male {
    first_name: String,
    last_name: String,
    nickname: String,
    birth_date: Date,
    children: std::collections::HashSet<Box<dyn IPerson>>,
    hair: HairType,
    wife: Box<dyn IFemale>,
}
impl IPerson for Male {
    fn first_name(&self) -> &String {
        &self.first_name
    }
    fn last_name(&self) -> &String {
        &self.last_name
    }
    fn nickname(&self) -> &String {
        &self.nickname
    }
    fn birth_date(&self) -> &Date {
        &self.birth_date
    }
    fn children(&self) -> &std::collections::HashSet<Box<dyn IPerson>> {
        &self.children
    }
    fn hair(&self) -> &HairType {
        &self.hair
    }
}
impl IMale for Male {
    fn wife(&self) -> &Box<dyn IFemale> {
        &self.wife
    }
}
