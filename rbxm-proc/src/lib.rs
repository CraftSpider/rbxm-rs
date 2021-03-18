
use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr, Ident};

fn match_path(path: &syn::Path, ident: &str) -> bool {
    path.is_ident(&Ident::new(ident, Span::call_site().into()))
}

fn path_as_ident(path: &syn::Path) -> String {
    path.segments.last().unwrap().ident.to_string()
}

fn to_pascal_case(ident: &Ident) -> LitStr {
    let ident = ident
        .to_string()
        .split('_')
        .into_iter()
        .map(|segment| {
            let mut v = segment.chars().collect::<Vec<_>>();
            v[0] = v[0].to_uppercase().nth(0).unwrap();
            v.into_iter().collect::<String>()
        })
        .collect::<String>();
    LitStr::new(&ident, Span::call_site().into())
}

#[proc_macro_derive(FromProperty, attributes(delegate, propname, propty))]
pub fn from_property(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = &item.ident;

    let fields = match &item.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("FromProperty not supported on non-structs"),
    };
    let named_fields = match fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("FromProperty requires named fields"),
    };

    let constructor = named_fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let delegate = field.attrs.iter().any(|attr| match_path(&attr.path, "delegate"));
            let prop_name = field.attrs.iter().find(|attr| match_path(&attr.path, "propname")).map(|attr| {
                let meta = if let syn::Meta::NameValue(value) = attr.parse_meta().unwrap() {
                    value
                } else {
                    panic!()
                };
                if let syn::Lit::Str(lit) = meta.lit {
                    lit
                } else {
                    panic!()
                }
            }).unwrap_or(to_pascal_case(&field.ident.as_ref().unwrap()));

            let getter = if delegate {
                let field_ty = &field.ty;
                quote!(<#field_ty as FromProperty>::from_properties(properties)?)
            } else {
                let prop_ty = field.attrs.iter().find(|attr| match_path(&attr.path, "propty")).map(|attr| {
                    let meta = if let syn::Meta::NameValue(value) = attr.parse_meta().unwrap() {
                        value
                    } else {
                        panic!()
                    };
                    if let syn::Lit::Str(lit) = meta.lit {
                        lit.parse::<Ident>().unwrap()
                    } else {
                        panic!()
                    }
                }).unwrap_or_else(|| {
                    match &field.ty {
                        syn::Type::Path(path) => {
                            let path = path_as_ident(&path.path);
                            let name = match &*path {
                                "bool" => "Bool",
                                "i32" => "Int32",
                                "i64" => "Int64",
                                "f32" => "Float",
                                "f64" => "Double",
                                "String" => "TextString",
                                "Vec" => "BinaryString",
                                "Weak" => "InstanceRef",
                                ident => ident,
                            };
                            syn::Ident::new(name, Span::call_site().into())
                        },
                        _ => panic!("Unsupported type for property")
                    }
                });

                quote!(chomp_prop!(properties, #prop_name, #prop_ty))
            };

            quote!(#field_name: #getter)
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl FromProperty for #item_name {
            fn from_properties(properties: &mut std::collections::HashMap<String, Property>) -> std::result::Result<Self, crate::SerdeError> {
                Ok(Self {
                    #(#constructor),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToProperty)]
pub fn to_property(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    let name = item.ident;

    let expanded = quote! {
        impl ToProperty for #name {

        }
    };

    TokenStream::from(expanded)
}
