use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

const LIB: &str = "tiny-set";

/// Rewrite a repr-u* enum adding bit-flag discriminators to each variant
/// and create a compact (same repr-u* sized) set-like structure to represent
/// a combination of the enum variants.
#[proc_macro_attribute]
pub fn with_tiny_set(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(item as syn::DeriveInput);

    try_tiny_set(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn try_tiny_set(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    Ok(proc_macro2::TokenStream::from_iter([
        try_rewrite_enum(input)?,
        try_write_set(input)?,
    ]))
}

fn try_rewrite_enum(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let repr_uint = get_repr_uint(&input)?;
    let data = get_enum(&input)?;

    let attrs = &input.attrs;
    let vis = &input.vis;
    let enum_name = &input.ident;

    let variant_count = data.variants.len();
    if variant_count > repr_uint.width() {
        return Err(syn::Error::new_spanned(
            repr_uint,
            format!("{LIB}: the repr-type is not wide enough to accommodate all the variants"),
        ));
    }

    let mut variants = Vec::with_capacity(variant_count);
    let mut i = 0usize;
    for var in &data.variants {
        let attrs = &var.attrs;
        let name = &var.ident;

        if !var.fields.is_empty() {
            return Err(syn::Error::new_spanned(
                var,
                format!("{LIB}: a variant must not have fields"),
            ));
        }

        if var.discriminant.is_some() {
            return Err(syn::Error::new_spanned(
                var,
                format!("{LIB}: a variant must not have a discriminant"),
            ));
        }

        variants.push(quote! {
            #(#attrs)*
            #name = 1 << #i
        });

        i += 1;
    }

    Ok(quote! {
        #(#attrs)*
        #vis enum #enum_name {
            #(#variants,)*
        }
    })
}

fn try_write_set(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let repr_uint = get_repr_uint(&input)?;
    let data = get_enum(&input)?;

    let vis = &input.vis;
    let enum_name = &input.ident;
    // TODO: Better use the attribute span (think of go-to-definition) for the generated types.
    let set_name = syn::Ident::new(&format!("{enum_name}Set"), enum_name.span());
    let set_iter_name = syn::Ident::new(&format!("{set_name}Iter"), enum_name.span());
    let variant_count = data.variants.len();
    let variants = data.variants.iter().map(|var| var.ident.clone());

    Ok(quote! {
        #[derive(PartialEq, Eq, Copy, Clone, Default)]
        #vis struct #set_name(#repr_uint);

        impl #set_name {
            pub fn with(&self, item: #enum_name) -> Self {
                Self(self.0 | item as #repr_uint)
            }

            pub fn contains(&self, item: #enum_name) -> bool {
                self.0 & item as #repr_uint == item as #repr_uint
            }
        }

        impl core::iter::FromIterator<#enum_name> for #set_name {
            fn from_iter<T: core::iter::IntoIterator<Item = #enum_name>>(iter: T) -> Self {
                let mut set = Self::default();

                for item in iter {
                    set = set.with(item);
                }

                set
            }
        }

        impl core::iter::IntoIterator for #set_name {
            type Item = #enum_name;
            type IntoIter = #set_iter_name;

            fn into_iter(self) -> Self::IntoIter {
                #set_iter_name::new(self)
            }
        }

        impl core::fmt::Debug for #set_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_set().entries(self.into_iter()).finish()
            }
        }

        #vis struct #set_iter_name {
            set: #set_name,
            offset: usize,
        }

        impl #set_iter_name {
            fn new(set: #set_name) -> Self {
                Self { set, offset: 0 }
            }

            const VARIANTS: [#enum_name; #variant_count] = [
                #(#enum_name::#variants,)*
            ];
        }

        impl core::iter::Iterator for #set_iter_name {
            type Item = #enum_name;

            fn next(&mut self) -> core::option::Option<Self::Item> {
                let result = Self::VARIANTS
                    .get(self.offset)
                    .copied()
                    .filter(|&var| self.set.contains(var));

                self.offset += 1;

                result
            }
        }
    })
}

fn get_repr_uint(input: &syn::DeriveInput) -> syn::Result<ReprUInt> {
    let mut repr_uint_values = Vec::default();

    for attr in &input.attrs {
        if attr.path().is_ident("repr") {
            repr_uint_values.push(attr.parse_args::<ReprUInt>()?);
        }
    }

    match repr_uint_values.len() {
        0 => Err(syn::Error::new_spanned(
            input,
            format!("{LIB}: must have a `#[repr(u*)]` attribute"),
        )),
        1 => Ok(repr_uint_values.remove(0)),
        _ => Err(syn::Error::new_spanned(
            input,
            format!("{LIB}: must have a exactly one `#[repr(u*)]` attribute"),
        )),
    }
}

fn get_enum(input: &syn::DeriveInput) -> syn::Result<&syn::DataEnum> {
    use syn::Data::*;

    match &input.data {
        Enum(data) => Ok(data),
        Struct(_) | Union(_) => Err(syn::Error::new_spanned(
            input,
            format!("{LIB}: must be an enum"),
        )),
    }
}

struct ReprUInt {
    ident: syn::Ident,
    width: usize,
}

impl ReprUInt {
    fn width(&self) -> usize {
        self.width
    }
}

impl quote::ToTokens for ReprUInt {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.ident.to_tokens(tokens)
    }
}

impl syn::parse::Parse for ReprUInt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const UINT_TYPES: [(&str, u32); 6] = [
            ("u8", u8::BITS),
            ("u16", u16::BITS),
            ("u32", u32::BITS),
            ("u64", u64::BITS),
            ("u128", u128::BITS),
            ("usize", usize::BITS),
        ];

        let ident = input.parse::<syn::Ident>()?;

        if let Some(&(_, w)) = UINT_TYPES.iter().find(|(type_name, _)| ident.eq(type_name)) {
            Ok(Self {
                ident,
                width: w as usize,
            })
        } else {
            Err(syn::parse::Error::new_spanned(
                ident,
                format!(
                    "{LIB}: ident must be one of {}",
                    UINT_TYPES.map(|(type_name, _)| type_name).join(", ")
                ),
            ))
        }
    }
}
