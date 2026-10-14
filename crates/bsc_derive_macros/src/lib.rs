use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, GenericParam, Generics, Ident, Index,
};

#[proc_macro_derive(SteppedKeyframe)]
pub fn derive_stepped(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl SteppedKeyframe<#ident> for #ident {}
    };
    output.into()
}

#[proc_macro_derive(PulseKeyframe)]
pub fn derive_pulse(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl PulseKeyframe<#ident> for #ident {}
    };
    output.into()
}

#[proc_macro_derive(LinearKeyframe, attributes(skip_field))]
pub fn derive_linear(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);

    // Add a bound of Add, Sub, and Mul to each input
    let mut generics = generics;
    add_trait_bounds(&mut generics);

    let new_self = new_self_lerped(&data, &ident);

    let output = quote! {
        impl LinearKeyframe<#ident> for #ident {
            fn lerp(&self, next_frame_state: &#ident, ratio: f64) -> #ident {
                 #new_self
            }
        }
    };
    output.into()
}

/*
#ident{
    #(#field_names: (self.#field_names as f64 + (next_frame_state.#field_names as f64 - self.#field_names as f64) * ratio) as #field_types,)*
}
*/

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(generics: &mut Generics) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(parse_quote!(std::ops::{Add, Sub, Mul}));
        }
    }
}

fn new_self_lerped(data: &Data, ident: &Ident) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => {
                let recurse = fields_named.named.iter().map(| f| {
                    let field_name = &&f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    quote! {
                        #field_name: (self.#field_name as f64 + (next_frame_state.#field_name as f64 - self.#field_name as f64) * ratio) as #field_type,
                    }

                });

                quote! {
                    #ident{
                        #(#recurse)*
                    }
                }
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let recurse = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    let field_type = &f.ty;
                    quote! {
                        (self.#index as f64 + (next_frame_state.#index as f64 - self.#index as f64) * ratio) as #field_type,
                    }

                });

                quote! {
                    #ident(
                        #(#recurse)*
                    )
                }
                .into()
            }
            syn::Fields::Unit => quote! {#ident}.into(),
        },
        Data::Enum(_) => {
            // Enums are very difficult to get right and require some decisions that are best left to the end user. Not implemented for now
            unimplemented!()
            // let recurse: proc_macro2::TokenStream = data_enum.variants.iter().map(|f| {
            //     // TODO - iter these and instead of building just the final variant, build a match statement for each and every variant
            //     // Each variant needs to build the left hand side Enum::A(_,_) =>
            //     // and the right hand side that actually constructs the enum
            //     let pattern: proc_macro2::TokenStream = match &f.fields {
            //         syn::Fields::Named(fields_named) => {
            //             let recurse = fields_named.named.iter().map(| f| {
            //                 let field_name = &&f.ident.as_ref().unwrap();
            //                 let field_type = &f.ty;
            //                 quote! {
            //                     #field_name: (self.#field_name as f64 + (next_frame_state.#field_name as f64 - self.#field_name as f64) * ratio) as #field_type,
            //                 }

            //             });

            //             let field_names = fields_named.named.iter().map(|field| {
            //                 quote!{#field.ident,}
            //             });

            //             quote! {
            //                 #ident::#f.ident {#(#field_names)*} => #f.ident{
            //                     #(#recurse)*
            //                 },
            //             }
            //         },
            //         syn::Fields::Unnamed(fields_unnamed) => {
            //             let recurse = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| {
            //             let index = Index::from(i);
            //             let field_type = &f.ty;
            //             quote! {
            //                 (self.#index as f64 + (next_frame_state.#index as f64 - self.#index as f64) * ratio) as #field_type,
            //             }
            //         });

            //             let field_names = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| {
            //                 let index = Index::from(i);

            //                 quote!{"field_"#index.to_string(),}
            //             });

            //         quote! {
            //             #ident::#f.ident (#(#field_names)*) => #f.ident(
            //                 #(#recurse)*
            //             ),
            //         }
            //         .into()
            //     },
            //         syn::Fields::Unit => quote! {#ident::#f.ident => #f,},
            //     };

            //     pattern
            // }).collect();
            // // match test {
            // //     Test::A => todo!(),
            // //     Test::B(_, _) => todo!(),
            // //     Test::C { count, float } => todo!(),
            // // }
            // quote! {
            //     match self{
            //         #recurse
            //     }
            // }
            // .into()
        }
        Data::Union(_) => {
            // Unions require unsafe blocks. Not implemented at this time
            unimplemented!()

            // let recurse = data_union.fields.named.iter().map(| f| {
            //             let field_name = &&f.ident.as_ref().unwrap();
            //             let field_type = &f.ty;
            //             quote! {
            //                 #field_name: (self.#field_name as f64 + (next_frame_state.#field_name as f64 - self.#field_name as f64) * ratio) as #field_type,
            //             }

            //         });

            // quote! {
            //     #ident{
            //         #(#recurse)*
            //     }
            // }
        }
    }
}

/*
*
* impl LinearKeyframe<BodyRotationPoint> for BodyRotationPoint {
    fn lerp(&self, next_frame_state: &BodyRotationPoint, ratio: f64) -> BodyRotationPoint {
        BodyRotationPoint {
            point_x: self.point_x + (next_frame_state.point_x - self.point_x) * ratio as f32,
            point_y: self.point_y + (next_frame_state.point_y - self.point_y) * ratio as f32,
        }
    }
}
*
*/
