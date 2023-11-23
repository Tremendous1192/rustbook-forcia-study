// Import necessary libraries from the `proc_macro` module and `std`
use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::str::FromStr;

// Define a procedural macro named `tomlstruct`
#[proc_macro]
pub fn tomlstruct(input: TokenStream) -> TokenStream {
    // Initialize an empty string to build the resulting struct definition
    let mut ret = String::from("");

    // Iterate over tokens in the input `TokenStream`
    for token in input {
        // Match the type of each token
        match &token {
            // If the token is a group (enclosed in curly braces)
            TokenTree::Group(x) => {
                // Get the name of the struct from the group
                let name = get_struct_name(x).unwrap();
                // Build the struct definition string
                if ret == "" {
                    ret = format!("struct {} {{", name);
                } else {
                    ret = format!("{}}}\nstrcut {} {{", ret, name);
                }
            }

            // If the token is an identifier (e.g., field name)
            TokenTree::Ident(x) => {
                // Append the identifier to the struct definition string
                ret = format!("{}\n {}", ret, x.to_string());
            }

            // If the token is a literal (e.g., field type)
            TokenTree::Literal(x) => {
                // Check if the literal starts with a double quote (indicating a String)
                if x.to_string().starts_with('"') {
                    ret = format!("{}: String,", ret);
                } else {
                    ret = format!("{}: f64,", ret);
                }
            }
            _ => {},
        }
    }

    // Complete the struct definition string
    ret = format!("{}\n}}", ret);

    // Parse the resulting string into a `TokenStream` and return it
    FromStr::from_str(&ret).unwrap()
}

// Function to extract the struct name from a group of tokens
fn get_struct_name(input: &Group) -> Option<String> {
    // Check if the group is delimited by curly braces
    match input.delimiter() {
        Delimiter::Bracket => {
            // Iterate over tokens in the group
            for token in input.stream() {
                // If the token is an identifier, return its string representation
                if let TokenTree::Ident(x) = token {
                    return Some(x.to_string());
                }
            }
        }
        _ => (),
    }

    // Return `None` if no struct name is found
    None
}
