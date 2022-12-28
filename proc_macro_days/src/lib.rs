
#![feature(proc_macro_internals)]
#![feature(proc_macro_span)]

use proc_macro::{Span};
use proc_macro2::{TokenStream, Ident};

use syn;
use syn::parse::{Parse, ParseStream};

use quote::quote;

use glob::glob;
use regex::Regex;
use std::collections::HashMap;

#[proc_macro]
pub fn import_days(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();

    let re = Regex::new(r".+(\d+)").unwrap();
    for entry in glob("./src/day*.rs").expect("Failed to read pattern") {
        match entry {
            Ok(path) => {
                let prefix = path.file_stem().unwrap().to_str().unwrap();
                let caps = re.captures(prefix);
                match caps {
                    Some(caps) => {
                        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                        let day = &format!("{}", prefix);
                        let day_padded = &format!("day{:0>2}", n);

                        stream.extend(format!("mod {};", day).parse::<TokenStream>().unwrap());
                        if n < 10 {
                            stream.extend(format!("use {} as {};", day, day_padded).parse::<TokenStream>().unwrap());
                        }
                    },
                    None => {
                        // don't generate anything
                        // println!("No captures for {}!", name);
                    },
                }
                
            },
            Err(e) => println!("{:?}", e),
        }
    }

    return proc_macro::TokenStream::from(stream);
}


#[proc_macro]
pub fn instantiate_days(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();
    
    // {let mut v: Vec<&dyn Day> = Vec::new(); ( v.push(&day01::Day01 {}); )* v}
    let mut block  = TokenStream::new();

    let re = Regex::new(r".+(\d+)").unwrap();
    for entry in glob("./src/day*.rs").expect("Failed to read pattern") {
        match entry {
            Ok(path) => {
                let prefix = path.file_stem().unwrap().to_str().unwrap();
                let caps = re.captures(prefix);
                match caps {
                    // v.push(&day01::Day01 {});
                    Some(caps) => {
                        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                        let day_padded = &format!("day{:0>2}", n);
                        let day_padded_upper = &format!("Day{:0>2}", n);
                        let instance = &format!("&{}::{} {{}}", day_padded, day_padded_upper).parse::<TokenStream>().unwrap();
                        block.extend(quote!{
                            v.push( #instance );
                        });
                    },
                    None => {
                        // don't generate anything
                        // println!("No captures for {}!", name);
                    },
                }
                
            },
            Err(e) => println!("{:?}", e),
        }
    }
    stream.extend(quote!{
        {
            let mut v: Vec<&dyn Day> = Vec::new();
            #block
            v
        }
    });

    return proc_macro::TokenStream::from(stream);
}


#[derive(Debug, Default)]
struct DayParser {
    parts: HashMap<usize, Ident>,
}
impl Parse for DayParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut day_parser = DayParser::default();

        let mut i = 0;
        while !input.is_empty() {
            i += 1;
            let fn_ident = input.parse::<Ident>()?;
            // Optional, Ok vs Err doesn't matter. Just consume if it exists.
            input.parse::<syn::token::Comma>().ok();
            day_parser.parts.insert(i, fn_ident);
        }

        return Ok(day_parser);
    }
}
// A macro designed to look like a regular function call rather than a TokenStream,
// aiming to replace the smallest possible amount of code for the use it's designed for.
// It only expands to a `struct Day#` implementing `Day`,
// effectively linking the passed functions to main.rs
// as main.rs has macros importing & instantiating Day# structs for each Day#.rs file.
//
// Example:
// fn part1(_input: &str) -> Result<isize, String> {
//     return Ok(0);
// }
// ...
// impl_day!( part1, ... );
#[proc_macro]
pub fn impl_day(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();
    
    let span = Span::call_site();
    let binding = span.source_file().path();
    let file = binding.to_str().unwrap();
    let re = Regex::new(r".*day(\d+).rs").unwrap();
    let caps = re.captures(file);
    if let Some(caps) = caps {
        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let day_padded_upper = format!("Day{:0>2}", n).parse::<TokenStream>().unwrap();

        let day_parser = syn::parse_macro_input!(input as DayParser);
        let mut trait_parts = TokenStream::new();

        for (k, fn_ident) in day_parser.parts.into_iter() {
            let trait_part_ident = format!("part_{}", k).parse::<TokenStream>().unwrap();
            trait_parts.extend(quote!{
                fn #trait_part_ident(&self, input: &str) -> Result<String, ()> {
                    return Ok(format!("Part {}: {:?}", #k, #fn_ident(input)));
                }
            });
        }
        // for (i, tt) in input.into_iter().enumerate() {
        //     if let proc_macro::TokenTree::Ident(fn_ident) = tt {
        //         let trait_part_ident = proc_macro::Ident::new(format!("part_{}", i).as_str(), span);
        //         trait_parts.extend(quote!{
        //             fn #trait_part_ident(&self, input: &str) -> Result<String, ()> {
        //                 return Ok(format!("Part {}: {:?}", #i, #fn_ident(input)));
        //             }
        //         });
        //     }
        // }
        
        stream.extend(quote!{
            #[derive(Debug)]
            pub struct #day_padded_upper {}

            impl Day for #day_padded_upper {
                #trait_parts
            }
        });

    } else {
        // don't generate anything
        let str = format!("Tried to implement Day for a file with malformed name: file = \"{}\" , re = \"{:?}\"", file, re);
        println!("{}", str);
        // compile_error!(str);
    }

    return proc_macro::TokenStream::from(stream);
}
