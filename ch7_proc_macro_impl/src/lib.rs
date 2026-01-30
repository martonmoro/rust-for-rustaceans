use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    ItemFn, LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro]
pub fn make_fn(_input: TokenStream) -> TokenStream {
    let fn_name = Ident::new("generated_fn", Span::call_site());

    let tokens = quote! {
        fn #fn_name() {
            println!("Hello from generated_fn");
        }
    };

    tokens.into()
}

#[proc_macro]
pub fn inc_var(input: TokenStream) -> TokenStream {
    let var_name = input.to_string();
    let ident = Ident::new(&var_name, Span::call_site());
    let tokens = quote! {
        #ident += 1;
    };

    tokens.into()
}

struct TimedArgs {
    unit: String,
}

impl Parse for TimedArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(TimedArgs {
                unit: "ms".to_string(),
            });
        }

        let unit_lit: LitStr = input.parse()?;
        let unit = unit_lit.value();

        match unit.as_str() {
            "ns" | "us" | "ms" | "s" => Ok(TimedArgs { unit }),
            _ => Err(syn::Error::new_spanned(
                unit_lit,
                "Invalid time unit. Use 'ns', 'us', 'ms', or 's'",
            )),
        }
    }
}

#[proc_macro_attribute]
pub fn timed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as TimedArgs);
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;

    let conversion = match args.unit.as_str() {
        "ns" => quote! { duration.as_nanos() },
        "us" => quote! { duration.as_micros() },
        "ms" => quote! { duration.as_millis() },
        "s" => quote! { duration.as_secs_f64() },
        _ => unreachable!(),
    };

    let unit = &args.unit;

    let output = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            let timed_value = #conversion;
            println!("{} took {}{}", stringify!(#fn_name), timed_value, #unit);
            result
        }
    };

    output.into()
}
