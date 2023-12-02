use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_str;
use syn::Type;

#[proc_macro]
pub fn gen_day_one_routes(_item: TokenStream) -> TokenStream {
    let mut token_stream_collection: Vec<proc_macro2::TokenStream> = Vec::new();
    for index in 2..20 {
        let function_ident = format_ident!("day_one_route_{}", index as usize);
        let idents: Vec<syn::Ident> = (1..(index + 1))
            .map(|i| format_ident!("num{}", i as usize))
            .collect();
        let path_tuple: Vec<syn::Type> = (0..index)
            .map(|_| parse_str::<Type>("usize").unwrap())
            .collect();

        let path_strs: Vec<String> = (1..(index + 1)).map(|i| format!("{{num{}}}", i)).collect();
        let path_str = format!("/1/{}", path_strs.join("/"));
        let token_stream = quote! {
            #[get(#path_str)]
            async fn #function_ident(path: actix_web::web::Path<(#(#path_tuple),*)>) -> HttpResponse
            {
                let (#(#idents),*) = path.into_inner();
                let xor_result = #(#idents)|*;
                let pow_result = usize::pow(xor_result, 3);
                return HttpResponse::Ok().body(format!("{}", pow_result));
            }
        };

        println!("{}", token_stream);
        token_stream_collection.push(token_stream);
    }

    let final_token_stream = quote! {
        #(#token_stream_collection)*
    };

    return TokenStream::from(final_token_stream);
}

