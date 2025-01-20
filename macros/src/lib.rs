use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident as Identifier};

#[proc_macro]
pub fn generate_terminal_tests(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("generate_terminal_tests takes no arguments!");
    }
    let tests = std::fs::read_dir("roms")
        .unwrap()
        .map(|entry| {
            let entry = entry?;
            let mut test_name = entry.file_name().into_string().unwrap().to_lowercase();
            test_name.truncate(test_name.len() - ".ch8".len());

            let interpreter_declaration = if test_name.contains("hires") {
                quote! { Interpreter::from(Interpreter::builder().display_width(128).display_height(64).build()) }
            } else {
                quote! { Interpreter::default() }
            };

            let test_name = Identifier::new(&test_name, proc_macro2::Span::call_site());

            let path = entry.path().to_string_lossy().to_string();

            Ok(quote! {
                    #[test]
                    fn #test_name() {{
                        const PATH: &str = #path;
                        let mut interpreter = #interpreter_declaration;
                        interpreter.load_program_from_path(PATH).unwrap();
                        execute_program_terminal(interpreter).unwrap();
                    }}
            })
        })
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    quote! { #(#tests)* }.into()
}

/// Derives getter methods for each field using the field name. Each getter will return a reference to the field
#[proc_macro_derive(Getters)]
pub fn generate_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;
    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("Getters can only be derived on structs");
    };

    let getters = fields.iter().map(|field| {
        let field_identifier = &field.ident;
        let field_type = &field.ty;

        quote! {
            pub fn #field_identifier(&self) -> &#field_type {
                &self.#field_identifier
            }
        }
    });

    quote! {
        impl #struct_identifier {
            #(#getters)*
        }
    }
    .into()
}

fn does_not_derive(input: &DeriveInput, derive_macro_name: &str) -> bool {
    input.attrs.iter().any(|attribute| {
        attribute.path().is_ident("derive")
            && attribute
                .parse_nested_meta(|metadata| {
                    metadata
                        .path
                        .is_ident(derive_macro_name)
                        .then_some(())
                        .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), ""))
                })
                .is_ok()
    })
}

/// Derives getter methods for each field using the field name. This can only be derived for structs that derive [Copy]
#[proc_macro_derive(CopyGetters)]
pub fn generate_copy_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if does_not_derive(&input, "Copy") {
        panic!("CopyGetters can only be derived on types that derive Copy")
    };

    let struct_identifier = &input.ident;
    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("CopyGetters can only be derived on structs");
    };

    let getters = fields.iter().map(|field| {
        let field_identifier = &field.ident;
        let field_type = &field.ty;

        quote! {
            pub fn #field_identifier(&self) -> #field_type {
                self.#field_identifier
            }
        }
    });

    quote! {
        impl #struct_identifier {
            #(#getters)*
        }
    }
    .into()
}

#[proc_macro_derive(Builder)]
pub fn generate_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;
    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("Getters can only be derived on structs");
    };

    let builder_identifier = Identifier::new(
        format!("{struct_identifier}Builder").as_str(),
        Span::call_site(),
    );

    let setters = fields.iter().map(|field| {
        let field_identifier = &field.ident;
        let field_type = &field.ty;

        quote! {
            pub const fn #field_identifier(self, value: #field_type) -> Self {
                Self(#struct_identifier {
                    #field_identifier: value,
                    ..self.0
                })
            }
        }
    });

    quote! {
        #[derive(Debug, Default, Clone, Copy)]
        pub struct #builder_identifier(#struct_identifier);
        impl #builder_identifier {
            #(#setters)*
        }
    }
    .into()
}
