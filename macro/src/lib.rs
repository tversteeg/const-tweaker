use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, AttributeArgs, Error, Expr, ItemConst, Type,
    Type::Reference,
};

type TokenStream2 = proc_macro2::TokenStream;

/// The metadata for rendering the web GUI.
#[derive(Debug, FromMeta)]
struct Metadata {
    #[darling(default)]
    min: Option<f64>,
    #[darling(default)]
    max: Option<f64>,
    #[darling(default)]
    step: Option<f64>,
}

impl Metadata {
    pub fn from_attributes(args: AttributeArgs) -> Result<Self, TokenStream> {
        match Metadata::from_list(&args) {
            Ok(v) => Ok(v),
            Err(e) => Err(TokenStream::from(e.write_errors())),
        }
    }
}

/// Convert a given type to a const_tweaker Field with metadata.
fn field_init(
    ty: &Type,
    metadata: Metadata,
    default_value: Expr,
) -> Result<TokenStream2, TokenStream> {
    if let Type::Path(type_path) = &*ty {
        match type_path.path.get_ident() {
            Some(type_ident) => {
                let min = metadata.min.unwrap_or(-1.0);
                let max = metadata.max.unwrap_or(1.0);
                let step = metadata.step.unwrap_or(0.1);

                match &*(type_ident.to_string()) {
                    "f64" => Ok(quote! {
                        const_tweaker::Field::F64 {
                            value: #default_value,
                            min: #min,
                            max: #max,
                            step: #step,

                            module: module_path!().to_string(),
                            file: concat!(file!(), ":", line!()).to_string(),
                        }
                    }),
                    "bool" => Ok(quote! {
                        const_tweaker::Field::Bool {
                            value: #default_value,

                            module: module_path!().to_string(),
                            file: concat!(file!(), ":", line!()).to_string(),
                        }
                    }),
                    "str" => Ok(quote! {
                        const_tweaker::Field::String {
                            value: #default_value.to_string(),

                            module: module_path!().to_string(),
                            file: concat!(file!(), ":", line!()).to_string(),
                        }
                    }),
                    _ => mismatching_type_error(&ty),
                }
            }
            None => mismatching_type_error(&ty),
        }
    } else {
        mismatching_type_error(&ty)
    }
}

/// Convert a given type to a const_tweaker Field type.
fn field_name(ty: &Type) -> Result<TokenStream2, TokenStream> {
    if let Type::Path(type_path) = &*ty {
        match type_path.path.get_ident() {
            Some(type_ident) => match &*(type_ident.to_string()) {
                "f64" => Ok(quote! { const_tweaker::Field::F64 }),
                "bool" => Ok(quote! { const_tweaker::Field::Bool }),
                "str" => Ok(quote! { const_tweaker::Field::String }),
                _ => mismatching_type_error(&ty),
            },
            None => mismatching_type_error(&ty),
        }
    } else {
        mismatching_type_error(&ty)
    }
}

/// The error message when there's a type mismatch.
fn mismatching_type_error<T>(ty: &Type) -> Result<T, TokenStream> {
    Err(TokenStream::from(
        Error::new(
            ty.span(),
            "expected bool, &str or f64, other types are not supported in const_tweaker (yet)",
        )
        .to_compile_error(),
    ))
}

/// Proc macro call but with a result, which allows the use of `?`.
fn tweak_impl(args: AttributeArgs, input: ItemConst) -> Result<TokenStream, TokenStream> {
    let name = input.ident;
    let init_name = format_ident!("{}_INIT", name);
    let ty = if let Reference(type_ref) = *input.ty {
        type_ref.elem
    } else {
        input.ty
    };
    let field_init = field_init(&*ty, Metadata::from_attributes(args)?, *input.expr)?;
    let field_name = field_name(&*ty)?;

    let result = quote! {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        #[derive(Copy, Clone)]
        pub struct #name {
            __private_field: ()
        }

        impl #name {
            pub fn get(&self) -> &'static #ty {
                // Insert the default value only the first time
                #init_name.call_once(|| {
                    const_tweaker::DATA.insert(concat!(module_path!(), "::", stringify!(#name)), #field_init);
                });

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

        // The setting of the field in the map is only done once
        static #init_name: std::sync::Once = std::sync::Once::new();
        // A static variable is created as an instance of the above defined struct
        static #name: #name = #name { __private_field: () };
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
