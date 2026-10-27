use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn task_entry(args: TokenStream, item: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return r#"compile_error!("task_entry takes no arguments");"#
            .parse()
            .unwrap();
    }

    let mut output: TokenStream = "::pico_lib::task_trampoline!".parse().unwrap();

    output.extend([TokenTree::Group(Group::new(Delimiter::Brace, item))]);

    return output;
}
