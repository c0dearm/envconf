extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Field, Fields, FieldsNamed, Ident, Lit, Meta,
    NestedMeta,
};

struct FieldArgs {
    env: Option<Lit>,
    default: Option<Lit>,
}

struct FieldInit {
    name: Ident,
    args: FieldArgs,
}

impl FieldInit {
    fn parse_env_and_default(&self, v: &Lit, d: &Lit) -> quote::__private::TokenStream {
        let name = self.name.clone();

        quote! {
            #name: if let Ok(r) = std::env::var(#v) {
                r.parse().map_err(|_| Error::EnvParse(#v, r))?
            } else {
                #d.to_string().parse().map_err(|_| Error::DefaultParse(stringify!(#name), stringify!(#d)))?
            }
        }
    }

    fn parse_env_only(&self, v: &Lit) -> quote::__private::TokenStream {
        let name = self.name.clone();

        quote! {
            #name: if let Ok(r) = std::env::var(#v) {
                r.parse().map_err(|_| Error::EnvParse(#v, r))?
            } else {
                return Err(Error::MissingEnv(#v));
            }
        }
    }

    fn parse_default_only(&self, d: &Lit) -> quote::__private::TokenStream {
        let name = self.name.clone();

        quote! { #name: #d.to_string().parse().map_err(|_| Error::DefaultParse(stringify!(#name), stringify!(#d)))? }
    }
}

impl quote::ToTokens for FieldInit {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let gen = if let Some(v) = &self.args.env {
            if let Some(d) = &self.args.default {
                self.parse_env_and_default(v, d)
            } else {
                self.parse_env_only(v)
            }
        } else if let Some(d) = &self.args.default {
            self.parse_default_only(d)
        } else {
            panic!("Either env or default attribute params are required")
        };

        tokens.extend(gen);
    }
}

fn parse_attribute_args(attr: &Attribute) -> FieldArgs {
    let mut args = FieldArgs {
        env: None,
        default: None,
    };

    match attr.parse_meta().unwrap() {
        Meta::List(list) => {
            for arg in list.nested {
                if let NestedMeta::Meta(Meta::NameValue(n)) = arg {
                    let name = n.path.segments.first().unwrap().ident.to_string();
                    if name == "env" {
                        args.env = Some(n.lit);
                    } else if name == "default" {
                        args.default = Some(n.lit);
                    } else {
                        panic!("Invalid attribute argument name {}", name)
                    }
                }
            }
        }
        _ => panic!("Couldn't parse attribute arguments"),
    };

    args
}

impl From<&Field> for FieldInit {
    fn from(f: &Field) -> FieldInit {
        let args = if let Some(attr) = f.attrs.first() {
            parse_attribute_args(attr)
        } else {
            panic!("Struct fields must have the conf attribute")
        };

        FieldInit {
            name: f.ident.clone().unwrap(),
            args,
        }
    }
}

fn impl_setting_struct(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let init_fields: Vec<FieldInit> = fields.iter().map(|x| x.into()).collect();

    let gen = quote! {
        impl Setting for #name {
            fn init<'a>() -> Result<Self, Error<'a>> {
                Ok(Self {
                    #(#init_fields),*
                })
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Setting, attributes(conf))]
pub fn setting_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let data = match ast.data {
        Data::Struct(d) => d,
        _ => panic!("Setting must be a struct"),
    };
    let fields = match data.fields {
        Fields::Named(FieldsNamed {
            brace_token: _,
            named,
        }) => named,
        _ => panic!("Struct fields must be named"),
    };

    impl_setting_struct(&name, &fields)
}
