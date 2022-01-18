extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, Span};
use syn::{DataStruct, DeriveInput, Field, Attribute};

enum GetterType {
    Ref,
    RefMut,
    Val
}

enum GsType {
    Getter(GetterType),
    Setter,
}

impl GsType {
    fn get_getter_type(&self) -> &GetterType {
        match self {
            Self::Getter(ref gt) => gt,
            Self::Setter => panic!("error occurred while creating getters and setters"),
        }
    }
}

struct Gs<'a> {
    ty: GsType,
    prefix: &'a str,
    suffix: &'a str,
    apply_to_all: bool
}

impl<'a> Gs<'a> {
    pub fn gen(self, ast: &DeriveInput) -> TokenStream2 {
        let name = &ast.ident;
        let generics = &ast.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
            let generated = fields
                .iter()
                .filter_map(|f| {
                    let token = match self.ty {
                        GsType::Getter(ref gt) => {
                            match gt {
                                GetterType::Ref => "get",
                                GetterType::RefMut => "get_mut",
                                GetterType::Val => "get_val",
                            }
                        },
                        GsType::Setter => "set"
                    };
                    // rust does actually try the conditions in order - the complier doesn't put the least expensive first,
                    // so check the boolean variable before calling the expensive function, for optimisation.
                    // see bottom of test file for benchmarks on this
                    if self.apply_to_all || has_tag(f.attrs.iter(), token) {
                        match self.ty {
                            GsType::Getter(_) => Some(self.gen_getter(f)),
                            GsType::Setter => Some(self.gen_setter(f)),
                        }
                    }else{
                        None
                    }
                })
                .collect::<Vec<_>>();

            quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#generated)*
                }
            }
        } else {
            panic!("This derive macro may not be used on enums");
        }
    }

    fn gen_setter(&self, field: &Field) -> TokenStream2 {
        let field_name = field.clone().ident.unwrap();
        let fn_name = Ident::new(&format!("set_{}", field_name), Span::call_site());
        let ty = field.ty.clone();
        quote! {
            #[inline(always)]
            pub fn #fn_name(&mut self, v: #ty) {
                self.#field_name = v;
            }
        }
    }

    fn gen_getter(&self, field: &Field) -> TokenStream2 {
        let field_name = field.clone().ident.unwrap();
        let ty = field.ty.clone();
        let fn_name = Ident::new(&format!("{}{}{}", self.prefix, field_name, self.suffix), Span::call_site());
        match self.ty.get_getter_type() {
            GetterType::Ref =>
                quote! {
                    #[inline(always)]
                    pub fn #fn_name(&self) -> &#ty {
                        &self.#field_name
                    }
                },
            GetterType::RefMut =>
                quote! {
                    #[inline(always)]
                    pub fn #fn_name(&mut self) -> &mut #ty {
                        &mut self.#field_name
                    }
                },
            GetterType::Val =>
                quote! {
                    #[inline(always)]
                    pub fn #fn_name(&self) -> #ty {
                        self.#field_name // this will cause an implicit copy so the type must implement the Copy (and Clone) traits
                    }
                }
        }
    }
}

#[proc_macro_derive(AddGetter, attributes(get))]
pub fn add_getter(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Getter(GetterType::Ref),
        prefix: "get_",
        suffix: "",
        apply_to_all: has_tag(ast.attrs.iter(), "get")
    };
    gs_builder.gen(&ast).into()
}

#[proc_macro_derive(AddGetterMut, attributes(get_mut))]
pub fn add_getter_mut(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Getter(GetterType::RefMut),
        prefix: "get_",
        suffix: "_mut",
        apply_to_all: has_tag(ast.attrs.iter(), "get_mut")
    };
    gs_builder.gen(&ast).into()
}

#[proc_macro_derive(AddGetterVal, attributes(get_val))]
pub fn add_getter_val(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Getter(GetterType::Val),
        prefix: "",
        suffix: "",
        apply_to_all: has_tag(ast.attrs.iter(), "get_val")
    };
    gs_builder.gen(&ast).into()
}

#[proc_macro_derive(AddSetter, attributes(set))]
pub fn add_setter(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gs_builder = Gs {
        ty: GsType::Setter,
        prefix: "set_",
        suffix: "",
        apply_to_all: has_tag(ast.attrs.iter(), "set")
    };
    gs_builder.gen(&ast).into()
}

/// Pass in an Iterator and a tag to search for in the meta data, and it will return weather the tag was found or not
fn has_tag<'a, T: Iterator<Item = &'a Attribute>>(mut attribs: T, tag_name: &str) -> bool {
    attribs
    .find_map(|v| {
        let meta = v.parse_meta().expect("failed to parse attr meta data");
        if meta.path().is_ident(tag_name) {
            Some(meta)
        } else {
            None
        }
    })
    .is_some()
}