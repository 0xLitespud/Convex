use crate::ast::*;

// 获取类型字符串
pub fn get_type_str(code_type: &Type) -> String {
    match code_type {
        Type::Int => String::from("int")
    }
}

// 获取一元运算符字符串
pub fn get_uo_str(uo: &UnaryOp) -> String {
    match uo {
        UnaryOp::Neg => String::from("-"),
        UnaryOp::LNot => String::from("!")
    }
}

// 获取表达式字符串
pub fn get_exp_str(exp: &Exp) -> String {
    return get_ue_str(&exp.ue);
}

// 获取一元运算表达式字符串
pub fn get_ue_str(ue: &UnaryExp) -> String {
    match ue {
        UnaryExp::Primary(pe) => {
            match pe {
                PrimaryExp::Exp(exp) => {
                    return format!("({})", get_exp_str(exp));
                },
                PrimaryExp::Number(num) => {
                    return num.to_string();
                }
            }
        },
        UnaryExp::Unary(uo, ue) => {
            return format!("{}{}", get_uo_str(uo), get_ue_str(ue));
        }
    }
}