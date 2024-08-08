// This is free and unencumbered software released into the public domain.

use crate::util::protoflow_crate;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn expand_derive_block(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let protoflow = protoflow_crate();
    let ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let _fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: fields, .. }),
            ..
        }) => fields.into_iter().collect(),
        Data::Struct(DataStruct {
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }),
            ..
        }) => fields.into_iter().collect(),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => Vec::new(),
        _ => panic!("`#[derive(Block)]` only supports structs"),
    };

    #[cfg(not(feature = "sysml"))]
    let impl_sysml_traits = quote! {};

    #[cfg(feature = "sysml")]
    let impl_sysml_traits = quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::prelude::sysml_model::BlockDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::PartDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::ItemDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::OccurrenceDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Definition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Structure for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Class for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Classifier for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Type for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Namespace for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Element for #ident #ty_generics #where_clause {}
    };

    Ok(quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::BlockDescriptor for #ident #ty_generics #where_clause {
            fn inputs(&self) -> #protoflow::prelude::Vec<#protoflow::PortDescriptor> {
                #protoflow::prelude::vec![] // TODO
            }

            fn outputs(&self) -> #protoflow::prelude::Vec<#protoflow::PortDescriptor> {
                #protoflow::prelude::vec![] // TODO
            }
        }

        #impl_sysml_traits
    })
}
