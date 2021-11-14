use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, LitStr};

fn match_path(path: &syn::Path, ident: &str) -> bool {
    path.is_ident(&Ident::new(ident, Span::call_site().into()))
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

fn has_attr(attrs: &Vec<syn::Attribute>, name: &str) -> bool {
    attrs.iter().any(|attr| match_path(&attr.path, name))
}

#[proc_macro_derive(Inherits)]
pub fn inherits(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = &item.ident;

    let fields = match &item.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Inherits not supported on non-structs"),
    };
    let named_fields = match fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Inherits requires named fields"),
    };

    let field = named_fields.named.first().unwrap();

    let (target_ty, target_name) = (&field.ty, field.ident.as_ref().unwrap());

    let expanded = quote!(
        impl core::ops::Deref for #item_name {
            type Target = #target_ty;

            fn deref(&self) -> &Self::Target {
                &self.#target_name
            }
        }

        impl core::ops::DerefMut for #item_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#target_name
            }
        }
    );

    TokenStream::from(expanded)
}

#[proc_macro_derive(
    PropertyConvert,
    attributes(delegate, shared, propname)
)]
pub fn property_convert(item: TokenStream) -> TokenStream {
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

    let (constructor, destructor): (Vec<_>, Vec<_>) = named_fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let delegate = has_attr(&field.attrs, "delegate");
            let shared = has_attr(&field.attrs, "shared");
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

            let (getter, setter) = if delegate {
                let field_ty = &field.ty;
                (
                    quote!(<#field_ty as FromProperty>::from_properties(properties)?),
                    quote!(<#field_ty as ToProperty>::to_properties(&self.#field_name, properties)),
                )
            } else {
                (
                    quote!(
                        crate::serde::internal::FieldFromProperties::from_properties(
                            crate::serde::internal::FieldAttrs { field_name: stringify!(#field_name), prop_name: #prop_name, shared: #shared },
                            properties,
                        )?
                    ),
                    quote!(
                        crate::serde::internal::FieldToProperties::to_properties(
                            self.#field_name.clone(),
                            crate::serde::internal::FieldAttrs { field_name: stringify!(#field_name), prop_name: #prop_name, shared: #shared },
                            properties,
                        );
                    ),
                )
            };

            (quote!(#field_name: #getter), quote!(#setter))
        })
        .unzip();

    let expanded = quote! {
        impl FromProperty for #item_name {
            fn from_properties(properties: &mut alloc::collections::BTreeMap<String, Property>) -> core::result::Result<Self, crate::SerdeError> {
                Ok(Self {
                    #(#constructor),*
                })
            }
        }

        impl ToProperty for #item_name {
            fn to_properties(&self, properties: &mut alloc::collections::BTreeMap<String, Property>) {
                #(#destructor;)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(EnumConvert)]
pub fn enum_convert(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = &item.ident;

    let variants = match &item.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("TryFrom not supported on non-enums"),
    };

    let mut last_discrim = 0;

    let variant_match = variants
        .iter()
        .map(|var| {
            let var_name = &var.ident;
            let discrim: i32 = var
                .discriminant
                .as_ref()
                .map(|(_, expr)| match expr {
                    syn::Expr::Lit(expr_lit) => {
                        if let syn::Lit::Int(val) = &expr_lit.lit {
                            val.base10_parse().unwrap()
                        } else {
                            panic!("Discriminant wasn't an integer literal")
                        }
                    }
                    _ => panic!("Discriminant wasn't a literal value"),
                })
                .unwrap_or_else(|| last_discrim + 1);

            last_discrim = discrim;

            quote!( #discrim => Ok(#item_name::#var_name) )
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl core::convert::TryFrom<i32> for #item_name {
            type Error = ();

            fn try_from(val: i32) -> core::result::Result<Self, ()> {
                match val {
                    #(#variant_match,)*
                    _ => Err(()),
                }
            }
        }

        #[allow(clippy::from_over_into)]
        impl core::convert::Into<i32> for #item_name {
            fn into(self) -> i32 {
                self as i32
            }
        }
    };

    TokenStream::from(expanded)
}
