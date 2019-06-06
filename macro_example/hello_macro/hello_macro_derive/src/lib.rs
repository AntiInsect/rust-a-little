extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {

        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    gen.into()
}

// #[proc_macro_derive(PrintBody)]
// pub fn print_body_derive(input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();

//     impl_print_body(&ast)
// }

// fn impl_print_body(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;

//     let gen = quote! {

//         impl PrintBody for #name {
//             fn print_body() {
//                 println!("{:?}", #name);
//             }
//         }
//     };

//     gen.into()
// }
