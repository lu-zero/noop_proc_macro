use noop_proc_macro::Serialize;
use noop_proc_macro::Deserialize;

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(default)]
    a: usize,
}

#[test]
fn test() {
    let _ = S { a: 0 };
}
