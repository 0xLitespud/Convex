mod ast;
mod codegen;
mod utils;

use codegen::generate_code;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

lalrpop_mod!(convex);

// 参数：convex [输入文件] -o [输出文件]
fn main() -> Result<()> {
    
    // 解析命令行参数
    let mut args = args();
    args.next();
    let input = args.next().unwrap();
    args.next();
    let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = convex::CompUnitParser::new().parse(&input).unwrap();

    // 生成代码
    let result = generate_code(&ast, true, &output);
    if let Err(e) = &result {
        println!("An error occurred while writing code file: \n\t{}", e);
        return result;
    }

    return Ok(());
}

