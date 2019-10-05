extern crate proc_macro;

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn serialize(_items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn deserialize(_items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

