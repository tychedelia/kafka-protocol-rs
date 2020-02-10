use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use syn::{parse_macro_input, Ident, Data, DeriveInput, Fields, Field, Meta, NestedMeta};
use quote::{quote};

fn decode_type(meta: Meta) -> TokenStream2 {
    match meta {
        Meta::Path(p) => quote!( franz_protocol::types::#p ),
        Meta::List(l) => {
            let p = l.path;
            let args: Vec<_> = l.nested.into_iter().map(|nested_meta| {
                if let NestedMeta::Meta(m) = nested_meta {
                    decode_type(m)
                } else {
                    panic!("Invalid attribute value.");
                }
            }).collect();

            quote!( franz_protocol::types::#p(#(#args),*) )
        },
        _ => panic!("Invalid attribute value."),
    }
}

struct Output {
    name: Ident,
    fields: Vec<FieldOutput>,
}

struct FieldOutput {
    name: Ident,
    type_: TokenStream2,
}

fn process_field(input: Field) -> FieldOutput {
    let name = input.ident.expect("Struct field should be named");
    let metas = input.attrs.into_iter()
        .filter(|attr| attr.path.is_ident("franz"))
        .flat_map(|attr| match attr.parse_meta().expect("Invalid attribute syntax") {
            Meta::List(list) => list.nested,
            _ => panic!("Invalid attribute syntax"),
        });
    
    
    let mut type_ = None;
    for nested_meta in metas {
        match nested_meta {
            NestedMeta::Meta(meta @ Meta::Path(_)) | NestedMeta::Meta(meta @ Meta::List(_)) => {
                assert!(type_.is_none(), "Cannot specify type more than once!");
                type_ = Some(decode_type(meta));
            },
            _ => panic!("Unexpected attribute value!"),
        }
    }

    let type_ = type_.expect("Must specify a type for all fields");

    FieldOutput {
        name,
        type_,
    }
}

fn process(input: DeriveInput) -> Output {
    let name = input.ident;

    let st = match input.data {
        Data::Struct(st) => st,
        _ => panic!("Can only derive `Encodable` on structs"),
    };

    let fields = match st.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Can only derive `Encodable` on structs with named fields"),
    };

    let fields = fields.into_iter().map(process_field).collect();

    Output {
        name,
        fields,
    }
}

#[proc_macro_derive(Encodable, attributes(franz))]
pub fn derive_encodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Output { name, fields } = process(input);

    let encode_lines = fields.into_iter().map(|FieldOutput { name, type_ }| {
        quote! { franz_protocol::Encoder::encode(&#type_, _buf, &self.#name)?; }
    });

    let result = quote! {
        impl franz_protocol::Encodable for #name {
            fn encode<B: bytes::BufMut>(&self, _buf: &mut B) -> Result<(), franz_protocol::EncodeError> {
                #(#encode_lines)*
                Ok(())
            }
        }
    };

    TokenStream::from(result)
}

#[proc_macro_derive(Decodable, attributes(franz))]
pub fn derive_decodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Output { name, fields } = process(input);

    let decode_lines = fields.into_iter().map(|FieldOutput { name, type_ }| {
        quote! { #name: franz_protocol::Decoder::decode(&#type_, _buf)?, }
    });

    let result = quote! {
        impl franz_protocol::Decodable for #name {
            fn decode<B: bytes::Buf>(_buf: &mut B) -> Result<Self, franz_protocol::DecodeError> {
                Ok(Self {
                    #(#decode_lines)*
                })
            }
        }
    };

    TokenStream::from(result)
}
