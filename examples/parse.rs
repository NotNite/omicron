fn main() {
    let temp = r#"
name B
extends A

# Specify variables with 'var <type> <name>'
# Built in types: byte, sbyte, short, ushort, int, uint, long, ulong, float, double, bool, string
var int my_cool_variable
var bool my_other_cool_variable

# Use attributes to specify additional information
[offset = "0x42069"] var C my_cool_struct

# Attributes can be on the same line, or the line before, a function or variable
[sig = "E8 ?? ?? ?? ??"]
func do_thing()

[vfunc = "1"] func do_another_thing()

# Arguments can be 'this', a type, or a struct
# Arguments can be pointers (*) and arrays ([])
func do_a_third_thing(this, C* a, int b)
func do_a_final_thing(this, int[] a)
"#;

    // Parse a string into a ParsedStruct
    let parsed = omicron::parse(temp.to_string()).unwrap();
    println!("{:#?}", parsed);

    // Convert a ParsedStruct into a (pretty-printed) JSON string
    let json = omicron::to_json(parsed);
    println!("{}", json);
}
