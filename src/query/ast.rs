pub enum Query<'a> {
    Select,
    Insert(u64, &'a str, &'a str),
}
