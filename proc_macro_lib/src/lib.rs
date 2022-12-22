
#![feature(proc_macro_internals)]
#![feature(proc_macro_span)]

use proc_macro;
use proc_macro2::{TokenStream, TokenTree, Group, Ident, Literal};

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
struct Part {
    ident: Option<usize>,
    ty: Option<syn::Type>,
    block: Option<Group>, // syn::token::Brace
    tests: Vec<Result<Group, Group>>, // syn::token::Paren
}

fn peek_past_type<T>(input: ParseStream, token: T) -> bool
where T: syn::parse::Peek
{
    let fork = input.fork();
    fork.parse::<syn::Type>().ok();

    return fork.peek(token);
}

#[derive(Debug)]
struct FuncToken {
    sig: syn::Signature,
    block: Group,
}
impl Parse for FuncToken {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return Ok( FuncToken{sig: input.parse()?, block: input.parse()?} );
    }
}
impl quote::ToTokens for FuncToken {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.sig.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

#[derive(Debug, Default)]
struct DayParser {
    main: Option<FuncToken>,
    parts: HashMap<usize, Part>,
}
impl Parse for DayParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut day_parser = DayParser::default();

        while !input.is_empty() {
            
            // fn main ...
            if let Ok(fn_main) = input.parse::<FuncToken>() {
                if fn_main.sig.ident == "main" {
                    day_parser.main = Some(fn_main);
                }
            }

            // part # type { expr* }
            let peek: Ident = input.fork().parse()?;
            if peek == "part" {
                input.parse::<Ident>()?;

                let id: Literal = input.parse()?;
                let n: usize = id.to_string().parse().unwrap();
                
                // useless & optional human clarity is allowed,
                // so we want to consume until we reach the required 'Type' token.
                let mut ty = syn::parse::<syn::Type>("&str".parse().unwrap()).unwrap();
                let error_position = input.lookahead1(); // more like 'lookbehind1'
                let mut found = false;
                while !input.is_empty() && !found {
                    // do we see `type group`?
                    let peek_block = peek_past_type(input, syn::token::Brace);
                    if peek_block {
                        ty = input.parse::<syn::Type>()?;
                    } else {
                        input.parse::<TokenTree>()?; // consume and ignore
                    }

                    found = input.peek(syn::token::Brace);
                }
                if input.is_empty() && !found {
                    // Unable to find expected tokens, error!
                    error_position.peek(syn::token::Type); // logs in error_message
                    error_position.peek(syn::token::Brace);
                    return Err(error_position.error());
                }

                let block: Group = input.parse()?;

                let part = day_parser.parts.entry(n).or_default();
                
                part.ident = Some(n);
                part.ty = Some(ty);
                part.block = Some(block);
            }
            
            // test # [assert](\(expr, expr\))*
            let peek: syn::Result<Ident> = input.fork().parse();
            if let Ok(peek) = peek {
                if peek == "test" {
                    input.parse::<Ident>()?;

                    let id: Literal = input.parse()?;
                    let n: usize = id.to_string().parse().unwrap();

                    let part = day_parser.parts.entry(n).or_default();

                    let lookahead = input.fork();
                    let mut valid_ahead = true;
                    while valid_ahead && !lookahead.is_empty() {
                        // ident is optional here `[Ident](Expr, Expr)`
                        // while `Ident="assert"` is used, user can use others and we need to consume & ignore them.
                        let mut had_ident = false;
                        let mut peek: Option<String> = None;
                        if let Ok(ident) = lookahead.fork().parse::<Ident>() {
                            peek = Some(ident.to_string());
                            lookahead.parse::<Ident>()?;
                            had_ident = true;
                        } else {
                        }
                        
                        // The test (Expr, Expr)
                        let test = lookahead.parse::<Group>();
                        
                        if let Ok(test) = test {
                            // Left: [in] part_impl(in), Right: identity
                            let block = if peek == Some("assert".to_owned()) {Err(test)} else {Ok(test)};

                            part.tests.push(block);

                            // consume to catch up to lookahead
                            if had_ident {
                                input.parse::<Ident>()?;
                            }
                            input.parse::<Group>()?;
                        } else {
                            // Not a valid test, assume user stopped writing tests.
                            // No need to consume as we were looking ahead.
                            valid_ahead = false;
                        }

                    }
                }
            }
        }

        return Ok(day_parser);
    }
}

#[proc_macro]
pub fn impl_day(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();
    
    let span = proc_macro::Span::call_site();
    let binding = span.source_file().path();
    let file = binding.to_str().unwrap();
    // let file = file!();
    let re = Regex::new(r".*day(\d+).rs").unwrap();
    let caps = re.captures(file);
    if let Some(caps) = caps {
        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let day_padded_upper = format!("Day{:0>2}", n).parse::<TokenStream>().unwrap();

        let day_parser = syn::parse_macro_input!(input as DayParser);

        let mut trait_part = TokenStream::new();
        let mut impl_part = TokenStream::new();
        let mut test_part = TokenStream::new();
        for (i, part) in day_parser.parts {
            let ty = part.ty;
            let block = part.block;

            let part_ident = format!("part_{}", i).parse::<TokenStream>().unwrap();
            let part_impl_ident = format!("part_impl_{}", i).parse::<TokenStream>().unwrap();

            trait_part.extend(quote!{
                fn #part_ident(&self, input: &str) -> Result<String, ()> {
                    return Ok(format!("Part {}: {:?}", #i, self.#part_impl_ident(input)));
                }
            });

            impl_part.extend(quote!{
                fn #part_impl_ident(&self, input: &str) -> Result<#ty , String>
                #block
            });

            let test_helper_ident = format!("test_helper_{}", i).parse::<TokenStream>().unwrap();
            let test_ident = format!("test_{}", i).parse::<TokenStream>().unwrap();
            let mut test_part_inner = TokenStream::new();
            for (test_n, test) in part.tests.iter().enumerate() {
                let test_ident = format!("{}_{}", test_ident, test_n).parse::<TokenStream>().unwrap();
                match test {
                    Ok(paren) => {
                        test_part_inner.extend(quote!{
                            #[test]
                            fn #test_ident() {
                                #test_helper_ident #paren;
                            }
                        });
                    }
                    Err(paren) => {
                        test_part_inner.extend(quote!{
                            #[test]
                            fn #test_ident() {
                                assert_eq! #paren;
                            }
                        });
                    }
                }
            }
            test_part.extend(quote!{
                fn #test_helper_ident(s: &str, v: #ty) {
                    assert_eq!(#day_padded_upper {}.#part_impl_ident(s).unwrap(), v);
                }

                #test_part_inner
            });
        }
        
        let main_func = day_parser.main;
        stream.extend(quote!{
            #main_func

            #[derive(Debug)]
            pub struct #day_padded_upper {}

            // impl Day_Better for #day_padded_upper {
            impl Day for #day_padded_upper {
                #trait_part
            }

            impl #day_padded_upper {
                #impl_part
            }

            #[cfg(test)]
            mod tests {
                use super::*;

                #test_part
            }
        });

    } else {
        // don't generate anything
        println!("Tried to implement Day for a file with malformed name: file = \"{}\" , re = \"{:?}\"", file, re);
    }

    // println!("return\n{}", proc_macro::TokenStream::from(stream.clone()).to_string());

    return proc_macro::TokenStream::from(stream);
}