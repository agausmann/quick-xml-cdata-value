use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Foo {
    #[serde(rename = "$value")]
    value: String,
}

const TEST_CASE: &[u8] = include_bytes!("test_case.xml");

fn main() {
    let foo_result: Result<Foo, _> = quick_xml::de::from_reader(TEST_CASE);
    println!("as Foo: {:?}", foo_result);

    let string_result: Result<String, _> = quick_xml::de::from_reader(TEST_CASE);
    println!("as bare string: {:?}", string_result);
}
