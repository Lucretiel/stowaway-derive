extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, WherePredicate};

#[proc_macro_derive(Stowable)]
pub fn derive_stowable(item: TokenStream) -> TokenStream {
    // Right now, we allow stowable for single-value structs. In the future,
    // we could allow it for any struct where the sum of the field sizes is the
    // same as the size of the struct itself, but it's not clear how to check
    // that in a derive macro. In principle we could use static_assertions,
    // but where would we actually put the assertion? How is it resolved in a
    // generic context? How do we make it give decent error messages?
    let mut input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let stowable = quote! { ::stowaway::Stowable };

    let field_type =
        match input.data {
            Data::Struct(ref struct_data) => {
                match struct_data.fields {
                    syn::Fields::Named(ref fields) => match fields.named.first() {
                        None => None,
                        Some(..) if fields.named.len() > 1 => return syn::Error::new(
                            fields.span(),
                            "derive(Stowable) can only be used on empty or single-field structs",
                        )
                        .to_compile_error()
                        .into(),
                        Some(field) => Some(&field.ty),
                    },
                    syn::Fields::Unnamed(ref fields) => match fields.unnamed.first() {
                        None => None,
                        Some(..) if fields.unnamed.len() > 1 => return syn::Error::new(
                            fields.span(),
                            "derive(Stowable) can only be used on empty or single-field structs",
                        )
                        .to_compile_error()
                        .into(),
                        Some(field) => Some(&field.ty),
                    },
                    syn::Fields::Unit => None,
                }
            }
            Data::Enum(enum_data) => {
                return syn::Error::new(
                    enum_data.enum_token.span(),
                    "derive(Stowable) is not currently supported for enums",
                )
                .to_compile_error()
                .into()
            }
            Data::Union(union_data) => {
                return syn::Error::new(
                    union_data.union_token.span(),
                    "derive(Stowable) is not currently supported for unions",
                )
                .to_compile_error()
                .into();
            }
        };

    if let Some(field_type) = field_type {
        let where_clause = input.generics.make_where_clause();
        let binding = quote! { #field_type: #stowable };
        let binding = TokenStream::from(binding);
        let binding = parse_macro_input!(binding as WherePredicate);
        where_clause.predicates.push(binding);
    }

    let (impl_clause, ty_clause, where_clause) = input.generics.split_for_impl();
    let definition = quote! {
        unsafe impl #impl_clause #stowable for #ident #ty_clause #where_clause {}
    };
    definition.into()
}
