fn main() {
    let temp = r#"
name B
extends A

[offset = "0x42069"] var C struct

[sig = "E8 ?? ?? ?? ??"]
func do_thing()

[vfunc = "3"]
func do_another_thing(this, C* a, int b)
"#;

    let parsed = omicron::parse(temp.to_string()).unwrap();
    println!("{:#?}", parsed);
    let json = omicron::to_json(parsed);
    println!("{}", json);
}
