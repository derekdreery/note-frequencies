extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn note_frequencies_32(toks: TokenStream) -> TokenStream {
    let middle_a = syn::parse_macro_input!(toks as syn::LitFloat).value();
    // we could be more accurate here probably, I'm assuming that floats don't screw us too much
    let notes = (0..128)
        .map(|idx| (middle_a * 2.0f64.powf((idx - 69) as f64 / 12.0)) as f32)
        .collect::<Vec<_>>();

    let out_toks = quote! {
        pub const NOTE_FREQUENCIES: [f32; 128] = [
            #(#notes),*
        ];
    };
    out_toks.into()
}
