mod generator;
mod codeutils;
mod converter;

use crate::ast::CompUnit;
use std::fs::write;
use self::generator::convex_to_c;

// declare: 为 true 则向 c 代码中添加函数声明
// path: 写入的文件名（相对路径，无后缀名）
pub fn generate_code(program: &CompUnit, declare: bool, path: &str) 
    -> Result<(), std::io::Error> 
{
    let c_codes = convex_to_c(program, declare);
    let c_file = path.to_string() + ".c";
    return write(c_file, c_codes);
}