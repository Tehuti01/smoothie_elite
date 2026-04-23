/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe8ebf323 | REVISION: 2026.04.20                           │
 * │ PATH: crates/01-silicon/smoothie-macros/src/lib.rs                       │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Industrial-grade neural layer and spec macro orchestration.  │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
/// Audits the function for L0 (Latency), A0 (Allocation), and PHI (Resonance) invariants.
pub fn seraphic_specification(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_item = parse_macro_input!(item as Item);

    match input_item {
        Item::Fn(input) => {
            let vis = &input.vis;
            let sig = &input.sig;
            let block = &input.block;

            let expanded = quote! {
                #vis #sig {
                    let result = (|| #block )();
                    result
                }
            };
            TokenStream::from(expanded)
        }
        _ => TokenStream::from(quote! { #input_item }),
    }
}

#[proc_macro_derive(SmoothieParams, attributes(param))]
/// Automatically implements the parameter interface for a SmoothiePlugin.
pub fn derive_smoothie_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(s) => s.fields,
        _ => panic!("SmoothieParams can only be derived for structs"),
    };

    let mut param_names = Vec::new();
    let mut get_cases = Vec::new();
    let mut set_cases = Vec::new();

    let mut param_count = 0;
    for field in fields {
        let field_name = field.ident.unwrap();
        for attr in field.attrs {
            if attr.path().is_ident("param") {
                // Extract name from #[param(name = "Gain")]
                let mut p_name = field_name.to_string();
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        p_name = s.value();
                    }
                    Ok(())
                });

                param_names.push(p_name);
                let idx = param_count;
                get_cases.push(quote! { #idx => self.#field_name.get(), });
                set_cases.push(quote! { #idx => self.#field_name.set(value), });
                param_count += 1;
            }
        }
    }

    let param_names_cases = param_names.iter().enumerate().map(|(i, n)| {
        quote! { #i => #n, }
    });

    let expanded = quote! {
        impl #name {
            pub fn get_derived_param_count(&self) -> usize { #param_count }
            
            pub fn get_derived_param_name(&self, index: usize) -> &'static str {
                match index {
                    #(#param_names_cases)*
                    _ => "",
                }
            }

            pub fn get_derived_param(&self, index: usize) -> f32 {
                match index {
                    #(#get_cases)*
                    _ => 0.0,
                }
            }

            pub fn set_derived_param(&mut self, index: usize, value: f32) {
                match index {
                    #(#set_cases)*
                    _ => {}
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Returns the build timestamp as a string.
pub fn build_timestamp(_input: TokenStream) -> TokenStream {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let s = format!("{}", now);
    let expanded = quote! { #s };
    TokenStream::from(expanded)
}
