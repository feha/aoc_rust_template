
#![feature(proc_macro_internals)]
#![feature(proc_macro_span)]

use proc_macro;
use proc_macro2::{TokenStream, TokenTree, Group, Ident, Literal, Punct};

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



// Surprisingly, compiler errors in the "pseudo code" of macro's is expressed properly by vscode.
// Example is changing the type of part 1's function, highlighting it's return value and the tests,
// giving the proper error about incorrect type.
//
// proc_macro_lib::impl_day!(
// 
// part 1
// (input: &str) -> &str { // <-- made to incorrectly expect &str
//     // println!("{}", input);
//     let solution = input.lines()
//         .map(|s| s.parse::<isize>().unwrap())
//         .fold(0, |sum, x| sum + x );
//     
//     return Ok(solution); // <-- 'solution' mismatched types: expected `&str`, found `isize`
// }
// 
// test 1
// assert("" , "")
// ("" , 0) // <-- '0' mismatched types: expected `&str`, found integer
// 
// );
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


#[derive(Debug, Default)]
struct Part2 {
    ident: Option<Ident>,
    // ty: Option<syn::Type>,
    // block: Option<Group>, // syn::token::Brace
    tests: Vec<Ident>,
}
#[derive(Debug, Default)]
struct DayParser2 {
    parts: HashMap<usize, Part2>,
}
impl Parse for DayParser2 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut day_parser = DayParser2::default();

        while !input.is_empty() {
            // 'part \d+ Ident' || 'test \d+ ident+'
            let pre = input.parse::<Ident>()?;
            let n = input.parse::<Literal>()?;
            // if let Ok(pre) = input.parse::<Ident>() { // "part"
                // if let Ok(n) = input.parse::<Literal>() {
                    let id = n.to_string().parse().expect(("Expected an integer, received ".to_owned() + n.to_string().as_str()).as_str());
                    let part = day_parser.parts.entry(id).or_default();
                    match pre.to_string().as_str() {
                        "part" => {
                            let fn_ident = input.parse::<Ident>()?;
                            part.ident = Some(fn_ident);
                            // if let Ok(fn_ident) = ident {
                            //     part.ident = Some(fn_ident);
                            // } else {
                            //     return Err(syn::Error::new(ident.span() , "This macro expected an Ident for this token: 'Ident Literal _token_'"));
                            // }
                        },
                        "test" => {
                            let mut not_a_prefix = true;
                            let lookahead = input.fork();
                            while !lookahead.is_empty() && not_a_prefix {
                                let ident = lookahead.parse::<Ident>()?;
                                // if let Ok(ident) = lookahead.parse::<Ident>() {
                                    match ident.to_string().as_str() {
                                        "part" => not_a_prefix = false,
                                        "test" => not_a_prefix = false,
                                        _ => {
                                            input.parse::<Ident>()?; // catch up to lookahead
                                            part.tests.push(ident);
                                        },
                                    }
                                // } else {
                                //     return Err("This macro expected an Ident for this token: 'Ident Literal _token_+'");
                                // }
                            }
                        },
                        _ => return Err(syn::Error::new(pre.span() , "This macro expected 'part' or 'test' for this token: '_token_ Literal Ident+'")),
                    }
                // } else {
                //     return Err("This macro expected a Literal (int) for this token: 'Ident _token_ Ident+'");
                // }
            // } else {
            //     return Err("This macro expected an Ident for this token: '_token_ Literal Ident+'");
            // }

        }

        return Ok(day_parser);
    }
}
// A version of the above, where the 'unnecessary' parts are not pseudo-coded as macro argument.
// Instead user writes proper functions, and instead pass 'references' (Ident's) to the functions.
// This should make IDE's have an easier time showing errors (even if current VSCode has no issues),
// and users able to code in a familiar environ & indentation.
// And overall lowers complexity so unexpected bugs should be less likely to exist.
//
// fn foo(_input: &str) -> Result<isize, String> {
//     return Ok(0);
// }
// fn bar_helper(s : & str, v : isize) {
//     assert_eq! (foo(s).unwrap(), v) ;
// }
// fn bar() {
//     assert_eq!("", "");
//     bar_helper("", 0);
// }
// proc_macro_lib::impl_day_2!(
//     part 1
//     foo
// 
//     test 1
//     bar
// );
#[proc_macro]
pub fn impl_day_2(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();
    
    let span = proc_macro::Span::call_site();
    let binding = span.source_file().path();
    let file = binding.to_str().unwrap();
    let re = Regex::new(r".*day(\d+).rs").unwrap();
    let caps = re.captures(file);
    if let Some(caps) = caps {
        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let day_padded_upper = format!("Day{:0>2}", n).parse::<TokenStream>().unwrap();

        let day_parser = syn::parse_macro_input!(input as DayParser2);

        let mut trait_part = TokenStream::new();
        let mut test_part = TokenStream::new();
        for (i, part) in day_parser.parts {
            // let part_impl_ident = format!("part_{}", i).parse::<TokenStream>().unwrap();
            let part_ident = part.ident;

            trait_part.extend(quote!{
                fn #part_ident(&self, input: &str) -> Result<String, ()> {
                    return Ok(format!("Part {}: {:?}", #i, #part_ident(input)));
                }
            });

            for (test_n, test) in part.tests.iter().enumerate() {
                let test_ident = format!("test_{}_{}", i, test_n).parse::<TokenStream>().unwrap();
                test_part.extend(quote!{
                    #[test]
                    fn #test_ident() {
                        #test()
                    }
                });
            }
        }
        
        stream.extend(quote!{
            #[derive(Debug)]
            pub struct #day_padded_upper {}

            impl Day for #day_padded_upper {
                #trait_part
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

#[derive(Debug, Default)]
struct DayParser3 {
    parts: HashMap<usize, Ident>,
}
impl Parse for DayParser3 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut day_parser = DayParser3::default();

        let mut i = 0;
        while !input.is_empty() {
            i += 1;
            let fn_ident = input.parse::<Ident>()?;
            input.parse::<syn::token::Comma>(); // Optional, Ok vs Err doesn't matter. Just consume if it exists.
            day_parser.parts.insert(i, fn_ident);
        }

        return Ok(day_parser);
    }
}
// A version of the above, where the macro is designed to look like a regular function call
// rather than a TokenStream, and simplified to replace the smallest possible amount of code.
// It only expands to a `struct Day#` implementing `Day`,
// effectively linking the passed functions to main.rs.
//
// fn part1(_input: &str) -> Result<isize, String> {
//     return Ok(0);
// }
// impl_day_3!( part1, ... );
#[proc_macro]
pub fn impl_day_3(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = TokenStream::new();
    
    let span = proc_macro::Span::call_site();
    let binding = span.source_file().path();
    let file = binding.to_str().unwrap();
    let re = Regex::new(r".*day(\d+).rs").unwrap();
    let caps = re.captures(file);
    if let Some(caps) = caps {
        let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let day_padded_upper = format!("Day{:0>2}", n).parse::<TokenStream>().unwrap();

        let day_parser = syn::parse_macro_input!(input as DayParser3);
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

    // println!("return\n{}", proc_macro::TokenStream::from(stream.clone()).to_string());

    return proc_macro::TokenStream::from(stream);
}



// Some old macros removed from utils.rs, precursor to the above.
// 
// files (and folders?) are implicit modules(?)
// "'mod'  looks for the 'foo' module in 'foo.rs' or 'foo/mod.rs'."
// mod utils {
// 
// #[macro_export]
// macro_rules! get_day {
//     ($day:ident, $day2:ident) => {
//         if include_optional::include_bytes_optional!("$day.rs").is_some() {Some(& $day :: $day2 {})} else {None}
//     }
// }
// 
// #[macro_export]
// macro_rules! test_helper {
//     (
//         $fn:ident ($in1:expr, $out1:expr)
//     ) => {
//         $fn($in1, $out1);
//     };
//     (
//         $fn:ident ( $in1:expr, $out1:expr, false )
//     ) => {
//         assert_eq!($in1, $out1);
//     };
// }
// #[macro_export]
// macro_rules! day {
//     (
//         $day:ident
//         part1 |$input1:ident $(: &str)?| -> $answer_type1:ident $part1_impl:block
//         part2 |$input2:ident $(: &str)?| -> $answer_type2:ident $part2_impl:block
//         test1 $( ( $test1:tt, $($test1_tail:tt)+ ) $(,)? )*
//         test2 $( ( $test2:tt, $($test2_tail:tt)+ ) $(,)? )*
//     ) => {
//         #[derive(Debug)]
//         pub struct $day {}
// 
//         impl Day for $day {
//             fn part1(&self, input: &str) {
//                 println!("part1: {:?}", self.part1_impl(input) );
//             }
//             
//             fn part2(&self, input: &str) {
//                 println!("part2: {:?}", self.part2_impl(input) );
//             }
//         }
//         
//         impl $day {
//             fn part1_impl(&self, $input1: &str) -> Result<$answer_type1, String> {
//                 $part1_impl
//             }
// 
//             fn part2_impl(&self, $input2: &str) -> Result<$answer_type1, String> {
//                 $part2_impl
//             }
//         }
//         
//         #[cfg(test)]
//         mod tests {
//         use super::*;
// 
//         fn test1(s: &str, v: $answer_type1) {
//             assert_eq!($day {}.part1_impl(s).unwrap(), v);
//         }
//         
//         #[test]
//         fn part1() {
//             $(
//                 crate::test_helper!( test1 ( $test1, $( $test1_tail )+ ) );
//             )*
//         }
// 
//         fn test2(s: &str, v: $answer_type2) {
//             assert_eq!($day {}.part2_impl(s).unwrap(), v);
//         }
//         
//         #[test]
//         fn part2() {
//             $(
//                 crate::test_helper!( test2 ( $test2, $( $test2_tail )+ ) );
//             )*
//         }
//         }
//     };
// }
