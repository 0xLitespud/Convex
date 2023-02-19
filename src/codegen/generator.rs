use crate::ast::*;
use crate::codegen::codeutils::Code;
use crate::codegen::converter::*;

// 将 convex 程序转换为 c
// declare: 为 true 则向 c 代码中添加函数声明
pub fn convex_to_c(program: &CompUnit, declare: bool) -> String {
    let mut c_code = Code::new();
    let main_func = &program.func_def;

    // 添加函数声明
    if declare {
        c_code.dump(
            format!(
                "{} {}();", get_type_str(&main_func.func_type), main_func.id
            )
        )
    }

    c_code.dump_in(
        format!(
            "{} {}() {{", get_type_str(&main_func.func_type), main_func.id
        )
    );

    let main_stmt = &main_func.block.stmt;
    c_code.dump(format!("return {};", get_exp_str(&main_stmt.exp)));
    c_code.dump_out("}".to_string());


    return c_code.code
}
