// const DATE_FMT: &str = "%a %b %d %H:%%M:%S %Y";
const DATE_FMT: &str = "%a %b %d %T %Y";

mod de;
mod error;
mod ser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
