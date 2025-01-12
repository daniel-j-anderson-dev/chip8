use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro]
pub fn generate_terminal_tests(_: TokenStream) -> TokenStream {
    let tests = std::fs::read_dir("roms")
        .unwrap()
        .map(|entry| {
            let entry = entry?;
            let mut test_name = entry.file_name().into_string().unwrap().to_lowercase();
            test_name.truncate(test_name.len() - ".ch8".len());

            let interpreter_declaration = if test_name.contains("hires") {
                quote! { Interpreter::builder().display_width(128).display_height(64).build() }
            } else {
                quote! { Interpreter::default() }
            };

            let test_name = Ident::new(&test_name, proc_macro2::Span::call_site());

            let path = entry.path().to_string_lossy().to_string();

            Ok(quote! {
                    #[test]
                    fn #test_name() {{
                        const PATH: &str = #path;
                        let mut interpreter = #interpreter_declaration;
                        interpreter.load_program_from_path(PATH).unwrap();
                        interpreter.execute_program_terminal();
                    }}
            })
        })
        .collect::<Result<Vec<_>, Box<dyn ::core::error::Error>>>()
        .unwrap();
    quote! { #(#tests)* }.into()
}
