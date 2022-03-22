panic!("This is not the module you are looking for, this is a placeholder to avoid name spoofing");

use proc_macro::TokenStream;

#[proc_macro]
pub fn fail(_item: TokenStream) -> TokenStream {
    panic!("This is not the module you are looking for, this is a placeholder to avoid name spoofing");
}

fail!();