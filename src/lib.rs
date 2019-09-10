extern crate proc_macro;
use proc_macro::TokenStream;

/// use a seperate directory under test_runs/ per test function
///
/// The directory is named module_name.function_name.
/// If no panic! during the test occurs, the test_runs/module_name.function_name
/// directory is removed.
#[proc_macro_attribute]
pub fn per_test_dir(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = input.sig.ident.to_string();
    let concatenated = format!("_pdf{}", fn_name);
    let varname = syn::Ident::new(&concatenated, input.sig.ident.span());
    let header = quote::quote! {
        let mut #varname = per_test_directory::PerTestDirectoryFixture::new(
            format!("{}.{}", {
                let mut s = module_path!().to_string();
                s.drain(..s.find("::").unwrap() + 2);
                s
            }, #fn_name.to_string()));
    };

    let footer = quote::quote! {
        #varname.passed = true;
    };

    input
        .block
        .stmts
        .insert(0, syn::parse(header.into()).unwrap());
    input.block.stmts.push(syn::parse(footer.into()).unwrap());

    let output = quote::quote! {
        #input
    };

    output.into()
}
