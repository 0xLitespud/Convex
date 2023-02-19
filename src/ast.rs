pub struct CompUnit {
    pub func_def: FuncDef,
}

pub struct FuncDef {
    pub func_type: Type,
    pub id: String,
    pub block: Block,
}

pub enum Type {
    Int
}

pub struct Block {
    pub stmt: Stmt
}

pub struct Stmt {
    pub exp: Exp
}

pub struct Exp {
    pub ue: UnaryExp
}

pub enum PrimaryExp {
    Exp(Box<Exp>), 
    Number(i32)
}

pub enum UnaryExp {
    Primary(PrimaryExp),
    Unary(UnaryOp, Box<UnaryExp>)
}

pub enum UnaryOp {
    Neg,
    LNot
}
