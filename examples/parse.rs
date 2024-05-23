use valid_npm_name::ValidName;

fn main() {
    let foo = ValidName::parse("foo").unwrap();
    assert_eq!("foo", foo.to_string());

    let bar_name = ValidName::parse("bar").unwrap();
    let bar = bar_name.as_ref();
    assert_eq!("bar", bar);

    let baz = ValidName::parse("baz").unwrap();
    assert_eq!("baz", baz.as_str());
}
