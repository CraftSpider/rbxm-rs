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
            if segment.len() == 0 {
                return String::new();
            }
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

#[proc_macro_derive(PropertyConvert, attributes(shared, propname))]
pub fn property_convert(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = &item.ident;

    let fields = match &item.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("PropertyConvert not supported on non-structs"),
    };
    let named_fields = match fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("PropertyConvert requires named fields"),
    };

    let (constructor, destructor): (Vec<_>, Vec<_>) = named_fields
        .named
        .iter()
        .map(|field| {
            let field_name = &field.ident;
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

            let (getter, setter) = (
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
            );

            (quote!(#field_name: #getter), quote!(#setter))
        })
        .unzip();

    let expanded = quote! {
        impl crate::serde::internal::FromProperties for #item_name {
            fn from_properties(properties: &mut alloc::collections::BTreeMap<alloc::string::String, crate::model::Property>) -> core::result::Result<Self, crate::SerdeError> {
                Ok(Self {
                    #(#constructor),*
                })
            }
        }

        impl crate::serde::internal::ToProperties for #item_name {
            fn to_properties(&self, properties: &mut alloc::collections::BTreeMap<alloc::string::String, crate::model::Property>) {
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

            quote!( #discrim => Ok(Self::#var_name) )
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

        impl core::convert::From<#item_name> for i32 {
            fn from(i: #item_name) -> Self {
                i as i32
            }
        }

        impl crate::serde::internal::FieldFromProperties for #item_name {
            fn from_properties(
                attrs: crate::serde::internal::FieldAttrs,
                properties: &mut alloc::collections::BTreeMap<alloc::string::String, crate::model::Property>
            ) -> crate::serde::error::Result<Self> {
                match properties.remove(attrs.prop_name) {
                    Some(crate::model::Property::Enum(val)) => Self::try_from(val)
                        .map_err(|_| crate::SerdeError::unknown_variant(val)),
                    Some(prop) => Err(crate::SerdeError::wrong_property_type(
                        alloc::string::String::from(attrs.prop_name),
                        Some((crate::model::property::PropertyType::Enum, prop.kind())),
                    )),
                    None => Err(crate::SerdeError::missing_property(alloc::string::String::from(attrs.prop_name))),
                }
            }
        }

        impl crate::serde::internal::FieldToProperties for #item_name {
            fn to_properties(
                self,
                attrs: crate::serde::internal::FieldAttrs,
                properties: &mut alloc::collections::BTreeMap<alloc::string::String, crate::model::Property>
            ) {
                properties.insert(alloc::string::String::from(attrs.prop_name), crate::model::Property::Enum(self.into()));
            }
        }
    };

    TokenStream::from(expanded)
}

struct InstanceResult {
    class_name: proc_macro2::TokenStream,
    name: proc_macro2::TokenStream,
    from_props: proc_macro2::TokenStream,
    to_props: proc_macro2::TokenStream,
}

impl InstanceResult {
    fn unzip(
        results: Vec<InstanceResult>,
    ) -> (
        Vec<proc_macro2::TokenStream>,
        Vec<proc_macro2::TokenStream>,
        Vec<proc_macro2::TokenStream>,
        Vec<proc_macro2::TokenStream>,
    ) {
        results.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |(mut classes, mut names, mut from_props, mut to_props), this| {
                classes.push(this.class_name);
                names.push(this.name);
                from_props.push(this.from_props);
                to_props.push(this.to_props);
                (classes, names, from_props, to_props)
            },
        )
    }
}

#[proc_macro_derive(InstanceExtra)]
pub fn instance_extra(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let item_name = &item.ident;

    let variants = match &item.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("InstanceExtra not supported on non-enums"),
    };

    let results = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            // Other is assumed to be the last variant in the enum
            if &variant_name.to_string() == "Other" {
                return InstanceResult {
                    class_name: quote!(#item_name::Other(class_name, _) => return class_name.clone()),
                    name: quote!(#item_name::Other(_, attrs) => {
                        if let Property::TextString(name) = attrs.get("Name").expect("Instance didn't have a name") {
                            name
                        } else {
                            panic!("Instance didn't have a Name")
                        }
                    }),
                    from_props: quote!(_ => {
                        // eprintln!("Other ty: {}", kind);
                        // eprintln!("    Properties: {:?}", properties);
                        let kind = Instance::Other(
                            String::from(kind),
                            properties
                                .iter()
                                .map(|(key, val)| (key.clone(), val.clone()))
                                .collect(),
                        );
                        properties.clear();
                        kind
                    }),
                    to_props: quote!(#item_name::Other(_, attrs) => properties.extend(attrs.clone())),
                };
            }

            let fields = if let syn::Fields::Unnamed(fields) = &variant.fields {
                fields
            } else {
                panic!("Expected unnamed enum fields");
            };

            let class_name_str = variant_name.to_string();
            let is_boxed = match &fields.unnamed[0].ty {
                syn::Type::Path(path) => path.path.segments[0].ident.to_string() == "Box",
                _ => panic!("Unexpected type in Variant"),
            };

            let class_name = quote!(#item_name::#variant_name(..) => #class_name_str);
            let name = quote!(#item_name::#variant_name(data) => &data.name);
            let from_props = if is_boxed {
                quote!(#class_name_str => #item_name::#variant_name(alloc::boxed::Box::new(#variant_name::from_properties(&mut properties)?)))
            } else {
                quote!(#class_name_str => #item_name::#variant_name(#variant_name::from_properties(&mut properties)?))
            };
            let to_props = quote!(#item_name::#variant_name(data) => data.to_properties(&mut properties));

            InstanceResult {
                class_name,
                name,
                from_props,
                to_props
            }
        })
        .collect();

    let (class_names, names, from_props, to_props) = InstanceResult::unzip(results);

    let expanded = quote! {
        impl #item_name {
            /// Get the name of the class for this kind
            #[must_use]
            pub fn class_name(&self) -> String {
                String::from(match self {
                    #(#class_names),*
                })
            }

            /// Get the name of this kind
            ///
            /// # Panics
            ///
            /// If the instance is of an unrecognized type which doesn't have a name.
            #[must_use]
            pub fn name(&self) -> &str {
                match self {
                    #(#names),*
                }
            }

            pub(crate) fn make_instance(kind: &str, mut properties: BTreeMap<String, Property>) -> Result<Instance, crate::SerdeError> {
                use crate::serde::internal::FromProperties;
                let out = match kind {
                    #(#from_props),*
                };

                if properties.is_empty() {
                    Ok(out)
                } else {
                    Err(crate::SerdeError::unconsumed_properties(
                        out.class_name(),
                        properties.into_iter().map(|(keys, _)| keys).collect(),
                    ))
                }
            }

            pub(crate) fn break_instance(&self) -> BTreeMap<String, Property> {
                use crate::serde::internal::ToProperties;
                let mut properties = BTreeMap::new();
                match self {
                    #(#to_props),*
                }
                properties
            }
        }
    };

    TokenStream::from(expanded)
}
