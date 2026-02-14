

extern crate proc_macro;

use std::sync::{Mutex, OnceLock};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitStr};

type HandlerFn = fn();

struct Operator {
    name: String,
    handler: HandlerFn,
}

#[proc_macro_attribute]
pub fn register_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_name = parse_macro_input!(attr as LitStr);
    let struct_ptr = parse_macro_input!(item as ItemStruct);

    let expanded = quote! {
        // Keep the original function
        #struct_ptr

        inventory::submit! {
            crate::Operator {
                name: #fn_name,
                handler: struct_ptr
            }
        }
    };

    expanded.into()
}