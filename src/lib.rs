extern crate proc_macro;
use proc_macro::TokenStream;
use quote::*;
use proc_macro2::Span;
use syn::*;
use syn::Meta::List;
use syn::Meta::Word;
use syn::punctuated::Pair::End;
use syn::NestedMeta::Meta;
use std::iter;


/// Derive a surjective ::core::convert::From<Unitary Enum Representation> conversion function
/// which maps all values which are not part of the enumeration to the last
/// enum discriminant.
#[proc_macro_derive(From)]
pub fn from(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse( input).unwrap();
    // get the representation of the enum
    let mut rep: Ident = Ident::new("isize",Span::call_site());
    if let Some( r) = ast.attrs.iter().find(|&a| a.path.is_ident("repr")) {
        if let Ok( List( l) ) = r.parse_meta() {
            if let Some( End( Meta( Word ( a) ) ) ) = l.nested.first() {
                if format!("{}",a) != "C" { // C -> isize                  
                    // an representation was given
                    rep = a.clone();
                }
            }
        }
    }
    // iteration over all fields
    let mut ret: TokenStream = quote!{}.into();
     
    if let Data::Enum( e) = ast.data {
        let mut prev_expr: Option<Expr> = None;
        let (names, discrs): (Vec<_>, Vec<_>) = e.variants.iter()
            .map(|x| {
                match x.fields {
                    Fields::Named(_) | Fields::Unnamed(_) =>
                        panic!("the enum's fields must \
                                be in the \"ident = discriminant\" form"),
                    Fields::Unit => ()
                }   
                let expr = match x.discriminant.as_ref() {
                    Some(discr) => discr.1.clone(),
                    None => match prev_expr {
                        Some(ref old_expr) => parse_quote!( 1 + #old_expr ),
                        None => parse_quote!( 0 ),
                    }
                };
                prev_expr = Some(expr.clone());
                ( x.ident.clone(), expr )
            }).unzip();
        let ty = ast.ident.clone();
        let vars_len = e.variants.len();
        let ty_repeat = iter::repeat(ty.clone()).take(vars_len);
        match names.last() {
            Some( last_name) => {
                let last_name = last_name.clone();
                ret = quote! {
                    impl From<#rep> for #ty {
                        fn from(x: #rep) -> Self {
                            match x {
                                #( x if x == #discrs => #ty_repeat::#names, )*
                                _ => #ty::#last_name 
                            }
                        }
                    }
                    impl From<#ty> for #rep {
                        fn from(x: #ty) -> Self { x as #rep }
                    }
                }.into();
            }
            _ => {}
        }
    } else {
        panic!("surjective_enum::From is only for enums");
    }
    ret
}



