// fn main() {
//     let ctx_lines = 2;
//     let search_items = "oo";
//     let haystack = "\
// Every face, every shop,
// bedroom window, public-house, and
// dark square is a picture
// feverishly turned--in search of what?
// It is the same with books.
// What do we seek
// through millions of pages?";

//     let mut tags: Vec<usize> = vec![]; // 保存已匹配行的行号

//     let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // 用于保存匹配行的上下若干行的信息

//     for (i, line) in haystack.lines().enumerate() {
//         if line.contains(search_items) {
//             tags.push(i);

//             let v = Vec::with_capacity(2 * ctx_lines + 1);
//             ctx.push(v);
//         }
//     }

//     if tags.is_empty() {
//         // 如果没有任何匹配行，提前结束
//         return;
//     }

//     for (i, line) in haystack.lines().enumerate() {
//         for (j, tag) in tags.iter().enumerate() {
//             let lower_bound = tag.saturating_sub(ctx_lines);
//             let upper_bound = tag + ctx_lines;

//             if (i >= lower_bound) && (i < upper_bound) {
//                 let line_as_string = String::from(line);
//                 let local_ctx = (i, line_as_string);
//                 ctx[j].push(local_ctx);
//             }
//         }
//     }

//     for local_ctx in ctx.iter() {
//         for &(i, ref line) in local_ctx.iter() {
//             // ref line 借用而不是移动
//             let line_num = i + 1;
//             println!("{}: {}", line_num, line);
//         }
//     }
// }

// use regex::Regex;

// fn main() {
//     let quote = "Every face, every shop, bedroom window, public-house, and
// dark square is a picture feverishly turned--in search of what?
// It is the same with books. What do we seek through millions of pages?";
//     let re = Regex::new("picture").unwrap();

//     for line in quote.lines() {
//         let contains_substring = re.find(line);

//         match contains_substring {
//             Some(_) => {
//                 print!("{}", line);
//             }
//             None => {}
//         }
//     }
// }

use clap::Parser;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "Misky", version = "1.0.0", about = "search for patterns", long_about = None)]
struct Args {
    /// The pattern to search for
    #[clap(short, required = true, takes_value = true, long, value_parser)]
    pattern: String,

    ///  File to search
    #[clap(short, required = false, takes_value = true, long, value_parser)]
    input: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let pattern = args.pattern;
    let input = args.input.unwrap_or_else(|| "-".to_string());

    let re = Regex::new(&pattern).unwrap();

    if input == "_" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();

        process_lines(reader, re);
    } else {
        let f = File::open(input)?;
        let reader = BufReader::new(f);

        process_lines(reader, re);
    }

    Ok(())
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line = line.unwrap();

        match re.find(&line) {
            Some(_) => {
                println!("{line}");
            }
            None => {}
        }
    }
}
