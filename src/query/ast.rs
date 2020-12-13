pub enum Query<'a> {
    Select,
    Insert(u32, &'a str, &'a str),
}
