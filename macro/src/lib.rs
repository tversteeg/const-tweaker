#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all
)]

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, Error, Expr, ItemConst, Type,
    Type::Reference,
};

type TokenStream2 = proc_macro2::TokenStream;

/// The metadata for rendering the web GUI.
#[derive(Debug, FromMeta)]
struct Metadata<T>
where
    T: FromMeta,
{
    #[darling(default)]
    min: Option<T>,
    #[darling(default)]
    max: Option<T>,
    #[darling(default)]
    step: Option<T>,
}

impl<T: FromMeta> Metadata<T> {
    pub fn from_attributes(args: AttributeArgs) -> Result<Self, TokenStream> {
        match Metadata::from_list(&args) {
            Ok(v) => Ok(v),
            Err(e) => Err(TokenStream::from(e.write_errors())),
        }
    }
}

/// Convert a given type to a const_tweaker Field with metadata.
fn field_init<T>(
    field_type: &str,
    ty: &Type,
    metadata: Metadata<T>,
    default_value: Expr,
    default_min: T,
    default_max: T,
    default_step: T,
) -> Result<TokenStream2, TokenStream>
where
    T: FromMeta + ToTokens,
{
    let min = metadata.min.unwrap_or(default_min);
    let max = metadata.max.unwrap_or(default_max);
    let step = metadata.step.unwrap_or(default_step);

    Ok(match field_type {
        "f32" => quote! {
            const_tweaker::Field::F32 {
                value: #default_value as f32,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "f64" => quote! {
            const_tweaker::Field::F64 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "i8" => quote! {
            const_tweaker::Field::I8 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "u8" => quote! {
            const_tweaker::Field::U8 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "i16" => quote! {
            const_tweaker::Field::I16 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "u16" => quote! {
            const_tweaker::Field::U16 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "i32" => quote! {
            const_tweaker::Field::I32 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "u32" => quote! {
            const_tweaker::Field::U32 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "i64" => quote! {
            const_tweaker::Field::I64 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "u64" => quote! {
            const_tweaker::Field::U64 {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "usize" => quote! {
            const_tweaker::Field::Usize {
                value: #default_value,
                min: #min,
                max: #max,
                step: #step,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "bool" => quote! {
            const_tweaker::Field::Bool {
                value: #default_value,

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        "str" => quote! {
            const_tweaker::Field::String {
                value: #default_value.to_string(),

                module: module_path!().to_string(),
                file: file!().to_string(),
                line: line!(),
            }
        },
        _ => {
            return mismatching_type_error(ty);
        }
    })
}

/// Convert a given type to a const_tweaker Field type.
fn field_name(field_type: &str, ty: &Type) -> Result<TokenStream2, TokenStream> {
    match field_type {
        "f32" => Ok(quote! { const_tweaker::Field::F32 }),
        "f64" => Ok(quote! { const_tweaker::Field::F64 }),
        "i8" => Ok(quote! { const_tweaker::Field::I8 }),
        "u8" => Ok(quote! { const_tweaker::Field::U8 }),
        "i16" => Ok(quote! { const_tweaker::Field::I16 }),
        "u16" => Ok(quote! { const_tweaker::Field::U16 }),
        "i32" => Ok(quote! { const_tweaker::Field::I32 }),
        "u32" => Ok(quote! { const_tweaker::Field::U32 }),
        "i64" => Ok(quote! { const_tweaker::Field::I64 }),
        "u64" => Ok(quote! { const_tweaker::Field::U64 }),
        "usize" => Ok(quote! { const_tweaker::Field::Usize }),
        "bool" => Ok(quote! { const_tweaker::Field::Bool }),
        "str" => Ok(quote! { const_tweaker::Field::String }),
        _ => mismatching_type_error(ty),
    }
}

/// Get the field type as a string.
fn field_type(ty: &Type) -> Result<String, TokenStream> {
    if let Type::Path(type_path) = &*ty {
        match type_path.path.get_ident() {
            Some(type_ident) => Ok(type_ident.to_string()),
            None => mismatching_type_error(ty),
        }
    } else {
        mismatching_type_error(ty)
    }
}

/// The error message when there's a type mismatch.
fn mismatching_type_error<T>(ty: &Type) -> Result<T, TokenStream> {
    Err(TokenStream::from(
        Error::new(
            ty.span(),
            "expected bool, &str, f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128 or usize, other types are not supported in const_tweaker (yet)",
        )
        .to_compile_error(),
    ))
}

/// Proc macro call but with a result, which allows the use of `?`.
fn tweak_impl(args: AttributeArgs, input: ItemConst) -> Result<TokenStream, TokenStream> {
    let name = input.ident;
    let init_name = format_ident!("{}_init", name);
    let ty = if let Reference(type_ref) = *input.ty {
        type_ref.elem
    } else {
        input.ty
    };
    let field_type = field_type(&*ty)?;
    let field_name = field_name(&field_type, &*ty)?;
    let field_init = match &*field_type {
        "f32" => field_init::<f32>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            0.0,
            1.0,
            0.001,
        )?,
        "f64" => field_init::<f64>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            0.0,
            1.0,
            0.001,
        )?,
        "i8" => field_init::<i8>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            i8::MIN,
            i8::MAX,
            1,
        )?,
        "u8" => field_init::<u8>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            u8::MIN,
            u8::MAX,
            1,
        )?,
        "i16" => field_init::<i16>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            i16::MIN,
            i16::MAX,
            1,
        )?,
        "u16" => field_init::<u16>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            u16::MIN,
            u16::MAX,
            1,
        )?,
        "i32" => field_init::<i32>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            i32::MIN,
            i32::MAX,
            1,
        )?,
        "u32" => field_init::<u32>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            u32::MIN,
            u32::MAX,
            1,
        )?,
        "i64" => field_init::<i64>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            i64::MIN,
            i64::MAX,
            1,
        )?,
        "u64" => field_init::<u64>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            u64::MIN,
            u64::MAX,
            1,
        )?,
        "usize" => field_init::<usize>(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            usize::MIN,
            usize::MAX,
            1,
        )?,
        "bool" => field_init(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            0,
            0,
            0,
        )?,
        "str" => field_init(
            &field_type,
            &*ty,
            Metadata::from_attributes(args)?,
            *input.expr,
            0,
            0,
            0,
        )?,
        _ => {
            return mismatching_type_error(&ty);
        }
    };

    let type_impls = if field_type == "str" {
        quote! {
            impl std::convert::From<#name> for &#ty {
                fn from(original: #name) -> &'static #ty {
                    original.get()
                }
            }
        }
    } else {
        quote! {
            impl std::convert::From<#name> for #ty {
                fn from(original: #name) -> #ty {
                    *original.get()
                }
            }
        }
    };

    let result = quote! {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        #[derive(Copy, Clone)]
        pub struct #name {
            __private_field: ()
        }

        impl #name {
            pub fn get(&self) -> &'static #ty {
                // Retrieve the value from the datastore and unwrap it
                match const_tweaker::DATA.get(concat!(module_path!(), "::", stringify!(#name))).expect("Value should have been added already").value() {
                    #field_name { ref value, .. } => unsafe {
                        // Make the reference static, so it leaks, but that shouldn't matter
                        // because there will always be one reference since the dashmap is global
                        std::mem::transmute::<&#ty, &'static #ty>(value as &#ty)
                    },
                    _ => panic!("Type mismatch, this probably means there's a duplicate value in the map, please report an issue")
                }
            }
        }

        // Automatically unwrap the primitive value from the struct when dereferencing
        impl std::ops::Deref for #name {
            type Target = #ty;

            fn deref(&self) -> &'static #ty {
                self.get()
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.get())
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.get())
            }
        }

        #type_impls

        // A static variable is created as an instance of the above defined struct
        static #name: #name = #name { __private_field: () };

        #[allow(non_snake_case)]
        #[const_tweaker::ctor]
        fn #init_name() {
            // Insert the value when the module is loaded
            const_tweaker::DATA.insert(concat!(module_path!(), "::", stringify!(#name)), #field_init);
        }
    };

    Ok(result.into())
}

/// Expose a const variable to the web GUI so it can be changed from a live setting.
#[proc_macro_attribute]
pub fn tweak(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemConst);

    match tweak_impl(args, input) {
        Ok(result) => result,
        Err(err) => err,
    }
}
