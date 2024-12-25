// Copyright (c) 2024 Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, Default, FromDeriveInput)]
#[darling(attributes(register), forward_attrs(allow, doc, cfg))]
struct RegisterAddressMacroArgs {
    address: u8,
    default: u8,
}

#[proc_macro_derive(RegisterAddress, attributes(register))]
pub fn derive_register_address(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let args = RegisterAddressMacroArgs::from_derive_input(&input).expect("Invalid arguments");
    let DeriveInput { ident, .. } = input;

    let default_value = args.default;
    let address_value = args.address;
    let address = quote! {
        const ADDRESS: u8 = #address_value;
    };

    let output = quote! {
        impl RegisterAddress for #ident {
            #address
        }

        impl Default for #ident {
            fn default() -> Self {
                Self(#default_value)
            }
        }

        impl From<#ident> for u8 {
            fn from(val: #ident) -> u8 {
                val.0
            }
        }

        impl From<u8> for #ident {
            fn from(val: u8) -> Self {
                Self(val)
            }
        }
    };

    output.into()
}

#[derive(Debug, Default, FromDeriveInput)]
#[darling(attributes(register), forward_attrs(allow, doc, cfg))]
struct RegisterFanMacroArgs {
    offset: u8,
    default: u8,
}

#[proc_macro_derive(RegisterOffset, attributes(register))]
pub fn derive_register_offset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let args = RegisterFanMacroArgs::from_derive_input(&input).expect("Invalid arguments");
    let DeriveInput { ident, .. } = input;

    let default_value = args.default;
    let offset_value = args.offset;
    let offset = quote! {
        const OFFSET: u8 = #offset_value;
    };

    let output = quote! {
        impl RegisterOffset for #ident {
            #offset
        }

        impl Default for #ident {
            fn default() -> Self {
                Self(#default_value)
            }
        }

        impl From<#ident> for u8 {
            fn from(val: #ident) -> u8 {
                val.0
            }
        }

        impl From<u8> for #ident {
            fn from(val: u8) -> Self {
                Self(val)
            }
        }
    };

    output.into()
}
