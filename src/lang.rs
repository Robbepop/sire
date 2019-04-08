use rustc::mir::BinOp;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuncDef {
    pub name: String,
    pub body: Expr,
    pub ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Int(usize),
    Uint(usize),
    Bool,
    Func(Vec<Ty>),
}

impl Ty {
    pub fn size(&self) -> Option<usize> {
        match self {
            Ty::Int(n) | Ty::Uint(n) => Some(*n),
            Ty::Bool => Some(8),
            Ty::Func(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Value(Value),
    Apply(Box<Expr>, Vec<Expr>),
    BinaryOp(BinOp, Box<Expr>, Box<Expr>),
    Switch(Box<Expr>, Vec<Expr>, Vec<Expr>),
    Nil,
}

impl Expr {
    pub fn replace(&mut self, target: &Self, substitution: &Self) {
        if *self == *target {
            *self = substitution.clone();
        } else {
            match self {
                Expr::Apply(e1, e2) => {
                    e1.replace(target, substitution);
                    for e in e2 {
                        e.replace(target, substitution);
                    }
                }
                Expr::Switch(e1, e2, e3) => {
                    e1.replace(target, substitution);
                    for e in e2 {
                        e.replace(target, substitution);
                    }
                    for e in e3 {
                        e.replace(target, substitution);
                    }
                }
                Expr::BinaryOp(_, e1, e2) => {
                    e1.replace(target, substitution);
                    e2.replace(target, substitution);
                }
                _ => (),
            }
        }
    }

    pub fn ty(&self) -> Ty {
        match self {
            Expr::Value(value) => value.ty(),
            Expr::Apply(e1, _) => match e1.ty() {
                Ty::Func(tys) => tys.first().unwrap().clone(),
                _ => unreachable!(),
            },
            Expr::BinaryOp(op, e1, _) => match op {
                BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => Ty::Bool,
                _ => e1.ty(),
            },
            Expr::Switch(_, _, es) => es.first().unwrap().ty().clone(),
            Expr::Nil => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Arg(usize, Ty),
    Const(u128, Ty),
    Function(String, Ty),
}

impl Value {
    pub fn ty(&self) -> Ty {
        match self {
            Value::Arg(_, ty) => ty,
            Value::Const(_, ty) => ty,
            Value::Function(_, ty) => ty,
        }
        .clone()
    }
}
