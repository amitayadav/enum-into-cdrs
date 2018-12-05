extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumIntoCDRSValue)]
pub fn enum_into_cdrs_macro_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_enum_into_cdrs_macro(&ast);
    gen.parse().unwrap()
}

fn impl_enum_into_cdrs_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        let conver_into_bytes: quote::Tokens = fields
            .iter()
            .map(|field| field.ident.clone().unwrap())
            .map(|field| {
                quote! {
                    let field_value = self.#field.enum_into_cdrs_macro_derive();
                    bytes.extend_from_slice(field_value.into_cbytes().as_slice());
                }
            })
            .fold(quote! {}, |acc, line| quote! {#acc #line});

        quote! {
        impl EnumIntoCDRSValue for #name {
            fn enum_into_cdrs_macro_derive(self) -> Value {
                    let mut bytes: Vec<u8> = vec![];
                    #conver_into_bytes
                    Bytes::new(bytes).into()
                }
            }
        }
    } else {
        panic!("#[derive(IntoCDRSValue)] is only defined for structs and enums, not for others!");
    }
}