//! This crate implements the macro for `blep` and should not be used directly.

use darling::{export::NestedMeta, FromAttributes, FromDeriveInput, FromMeta};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput, Lit};

#[derive(Debug, Clone, Copy)]
enum RenameStyle {
    CamelCase,
    PascalCase,
    KebabCase,
    SnakeCase,
}

impl FromMeta for RenameStyle {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "camel-case" => Ok(RenameStyle::CamelCase),
            "pascal-case" => Ok(RenameStyle::PascalCase),
            "kebab-case" => Ok(RenameStyle::KebabCase),
            "snake-case" => Ok(RenameStyle::SnakeCase),
            value => Err(darling::Error::custom(format!(
                "Unknown value for rename: {value:?}"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct TagPair {
    pub name: String,
    pub value: Lit,
}

impl FromMeta for TagPair {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        if items.len() != 2 {
            return Err(darling::Error::custom(format!(
                "Expected exactly two items in tag pair, found {}",
                items.len()
            )));
        }

        let name = match &items[0] {
            NestedMeta::Meta(_) => {
                return Err(darling::Error::custom(format!(
                    "Expected a string in first position of tag pair"
                )));
            }
            NestedMeta::Lit(x) => match x {
                Lit::Str(x) => x.value(),
                _ => {
                    return Err(darling::Error::custom(format!(
                        "Expected a string in first position of tag pair"
                    )))
                }
            },
        };

        let value = match &items[1] {
            NestedMeta::Meta(_) => {
                return Err(darling::Error::custom(format!(
                    "Expected a literal in second position of tag pair"
                )));
            }
            NestedMeta::Lit(x) => x.clone(),
        };

        Ok(TagPair { name, value })
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(codable))]
pub struct CodableAttrs {
    #[darling(multiple)]
    tag: Vec<TagPair>,
    rename: Option<RenameStyle>,
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(codable))]
pub struct CodableAttrAttrs {
    rename: Option<syn::LitStr>,
}

fn rename_input(style: RenameStyle, input: &str) -> String {
    match style {
        RenameStyle::CamelCase => input.to_lower_camel_case(),
        RenameStyle::PascalCase => input.to_pascal_case(),
        RenameStyle::KebabCase => input.to_kebab_case(),
        RenameStyle::SnakeCase => input.to_snake_case(),
    }
}

fn derive_encode_enum(
    data: DataEnum,
    attrs: CodableAttrs,
    input: DeriveInput,
) -> Result<TokenStream, syn::Error> {
    let variants: Vec<TokenStream> = data
        .variants
        .iter()
        .map(|x| {
            let local_attrs = CodableAttrAttrs::from_attributes(&x.attrs)?;

            let key = if let Some(rename) = local_attrs.rename {
                rename.value()
            } else if let Some(rename) = attrs.rename {
                rename_input(rename, &x.ident.to_string())
            } else {
                x.ident.to_string()
            };

            let value = &x.ident;

            Ok::<TokenStream, darling::Error>(quote! {
                Self::#value => #key
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let tags = attrs
        .tag
        .iter()
        .map(|x| {
            let TagPair { name, value } = x;

            quote! {
                c.encode(&#value, &#name)?
            }
        })
        .collect::<Vec<_>>();

    let enum_name = input.ident.clone();

    let output = if tags.is_empty() {
        quote! {
            impl ::codable::enc::Encode for #enum_name {
                fn encode<'e, E>(&self, encoder: &mut E) -> ::codable::enc::EncodeResult<'e, E>
                where
                    E: ::codable::enc::Encoder<'e>,
                {
                    use ::codable::enc::ValueContainer as _;

                    let mut c = encoder.as_value_container();
                    c.encode(&match self {
                        #(#variants),*
                    })?;
                    Ok(c.finish())
                }
            }
        }
    } else {
        quote! {
            impl ::codable::enc::Encode for #enum_name {
                fn encode<'e, E>(&self, encoder: &mut E) -> ::codable::enc::EncodeResult<'e, E>
                where
                    E: ::codable::enc::Encoder<'e>,
                {
                    use ::codable::enc::KeyedContainer as _;

                    let mut c = encoder.as_container();
                    #(#tags);* ;
                    c.encode(&match self {
                        #(#variants),*
                    }, &"value")?;
                    Ok(c.finish())
                }
            }
        }
    };

    Ok(output)
}

fn derive_encode_struct(
    data: DataStruct,
    attrs: CodableAttrs,
    input: DeriveInput,
) -> Result<TokenStream, syn::Error> {
    let is_tuple_struct = data.fields.iter().any(|x| x.ident.is_none());
    if is_tuple_struct {
        return Err(syn::Error::new_spanned(
            input,
            "tuple structs are not supported",
        ));
    }

    let fields = data
        .fields
        .iter()
        .map(|field| {
            let value = field.ident.clone().unwrap();
            let key = if let Some(rename) = attrs.rename {
                rename_input(rename, &value.to_string())
            } else {
                quote! { #value }.to_string()
            };

            let is_option = if let syn::Type::Path(ty) = &field.ty {
                let item = ty.path.segments.last().unwrap();
                item.ident.to_string() == "Option"
            } else {
                false
            };

            if is_option {
                quote! {
                    c.encode_option(self.#value.as_ref(), &#key)?
                }
            } else {
                quote! {
                    c.encode(&self.#value, &#key)?
                }
            }
        })
        .collect::<Vec<_>>();

    let tags = attrs
        .tag
        .iter()
        .map(|x| {
            let TagPair { name, value } = x;

            quote! {
                c.encode(&#value, &#name)?
            }
        })
        .collect::<Vec<_>>();

    let struct_name = input.ident.clone();
    let output = quote! {
        impl ::codable::enc::Encode for #struct_name {
            fn encode<'e, E>(&self, encoder: &mut E) -> ::codable::enc::EncodeResult<'e, E>
            where
                E: ::codable::enc::Encoder<'e>,
            {
                use ::codable::enc::KeyedContainer as _;

                let mut c = encoder.as_container();
                #(#tags);* ;
                #(#fields);* ;
                Ok(c.finish())
            }
        }
    };

    Ok(output)
}

#[doc(hidden)]
pub fn derive_encode(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(item)?;
    let attrs = CodableAttrs::from_derive_input(&input)?;

    match &input.data {
        syn::Data::Struct(x) => derive_encode_struct(x.clone(), attrs, input),
        syn::Data::Enum(x) => derive_encode_enum(x.clone(), attrs, input),
        syn::Data::Union(_) => todo!(),
    }
}

#[doc(hidden)]
pub fn derive_decode(_item: TokenStream) -> Result<TokenStream, syn::Error> {
    // Implement your proc-macro logic here. :)
    Ok(quote! {
        "Hello world!"
    })
}
