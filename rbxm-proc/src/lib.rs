
use proc_macro::{TokenStream, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr, Ident};

// TODO: Merge FromProperty/ToProperty? Name would probably be like PropertyConvert or something

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

    let field = named_fields
        .named
        .first()
        .unwrap();

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

#[proc_macro_derive(FromProperty, attributes(delegate, isenum, propname, propty))]
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
            let is_enum = field.attrs.iter().any(|attr| match_path(&attr.path, "isenum"));
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
            } else if is_enum {
                quote!({
                    let val = chomp_prop!(properties, #prop_name, Enum);
                    <i32 as core::convert::TryInto<_>>::try_into(val).map_err(|_| crate::SerdeError::UnknownVariant(val))?
                })
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

#[proc_macro_derive(ToProperty, attributes(delegate, isenum, propname, propty))]
pub fn to_property(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = item.ident;

    let fields = match &item.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("ToProperty not supported on non-structs"),
    };
    let named_fields = match fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("ToProperty requires named fields"),
    };

    let destructor = named_fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let delegate = field.attrs.iter().any(|attr| match_path(&attr.path, "delegate"));
            let is_enum = field.attrs.iter().any(|attr| match_path(&attr.path, "isenum"));
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
                quote!(<#field_ty as ToProperty>::to_properties(&self.#field_name, properties))
            } else if is_enum {
                quote!(properties.insert(#prop_name.to_string(), Property::Enum(self.#field_name.clone().into())))
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

                quote!(properties.insert(#prop_name.to_string(), Property::#prop_ty(self.#field_name.clone())))
            };

            quote!(#getter)
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl ToProperty for #item_name {
            fn to_properties(&self, properties: &mut std::collections::HashMap<String, Property>) {
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
            let discrim: i32 = var.discriminant.as_ref().map(|(_, expr)| match expr {
                syn::Expr::Lit(expr_lit) => if let syn::Lit::Int(val) = &expr_lit.lit {
                    val.base10_parse().unwrap()
                } else {
                    panic!("Discriminant wasn't an integer literal")
                }
                _ => panic!("Discriminant wasn't a literal value")
            }).unwrap_or_else(|| {
                last_discrim + 1
            });

            last_discrim = discrim;

            quote!( #discrim => Ok(#item_name::#var_name) )
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl std::convert::TryFrom<i32> for #item_name {
            type Error = ();

            fn try_from(val: i32) -> std::result::Result<Self, ()> {
                match val {
                    #(#variant_match,)*
                    _ => Err(()),
                }
            }
        }

        #[allow(clippy::from_over_into)]
        impl std::convert::Into<i32> for #item_name {
            fn into(self) -> i32 {
                self as i32
            }
        }
    };

    TokenStream::from(expanded)
}
