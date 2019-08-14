use crate::get_add_operation_fn_name;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma, Path, Result};

pub fn parse(routes: TokenStream) -> TokenStream {
    parse_inner(routes)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn parse_inner(routes: TokenStream) -> Result<TokenStream2> {
    let paths = <Punctuated<Path, Comma>>::parse_terminated.parse(routes)?;
    let add_operations = create_add_operations(paths.clone())?;
    Ok(quote! {
        {
            let settings = OpenApiSettings::new();
            let mut gen = OpenApiGenerator::new(settings.clone());
            #add_operations
            let spec = gen.into_openapi();

            let mut routes = ::rocket::routes![#paths];
            routes.push(ContentHandler::json(&spec).into_route(&settings.json_path));
            routes
        }
    })
}

fn create_add_operations(paths: Punctuated<Path, Comma>) -> Result<TokenStream2> {
    let function_calls = paths.into_iter().map(|path| {
        let fn_name = fn_name_for_add_operation(path.clone());
        let operation_id = operation_id(&path);
        quote! {
            #fn_name(&mut gen, #operation_id.to_owned())
                .expect(&format!("Could not generate OpenAPI operation for `{}`.", stringify!(#path)));
        }
    });
    Ok(quote! {
        #(#function_calls)*
    })
}

fn fn_name_for_add_operation(mut fn_path: Path) -> Path {
    let mut last_seg = fn_path.segments.last_mut().expect("syn::Path has segments");
    last_seg.value_mut().ident = get_add_operation_fn_name(&last_seg.value().ident);
    fn_path
}

fn operation_id(fn_path: &Path) -> String {
    let idents: Vec<String> = fn_path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    idents.join("_")
}
