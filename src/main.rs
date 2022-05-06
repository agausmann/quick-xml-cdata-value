use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Foo {
    #[serde(rename = "$value")]
    value: String,
}

fn main() {
    run_test("bad test case", b"<foo><![CDATA[<p>bar</p>]]></foo>");
    run_test("working test case", b"<foo>&lt;p&gt;bar&lt;/p&gt;</foo>");
}

fn run_test(name: &str, test_case: &[u8]) {
    let foo_result: Result<Foo, _> = quick_xml::de::from_reader(test_case);
    println!("{} as Foo: {:?}", name, foo_result);

    let string_result: Result<String, _> = quick_xml::de::from_reader(test_case);
    println!("{} as bare string: {:?}", name, string_result);

    println!();
}
