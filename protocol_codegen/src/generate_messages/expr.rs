use std::fmt;
use std::rc::Rc;

#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Precedence {
    Assignment,
    Range,
    LogicalOr,
    LogicalAnd,
    Comparison,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Shift,
    AddSub,
    MulDivMod,
    Cast,
    Unary,
    Suffix,
    Term,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RefType {
    Ref,
    Mut,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CmpType {
    Eq,
    Ne,
    Lt,
    Ge,
    Le,
    Gt,
}

impl CmpType {
    fn inverse(self) -> Self {
        use CmpType::*;
        match self {
            Eq => Ne,
            Ne => Eq,
            Lt => Ge,
            Ge => Lt,
            Le => Gt,
            Gt => Le,
        }
    }
}

impl fmt::Display for CmpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CmpType::*;
        f.write_str(match self {
            Eq => "==",
            Ne => "!=",
            Lt => "<",
            Ge => ">=",
            Le => "<=",
            Gt => ">",
        })
    }
}

#[derive(Debug, Clone)]
enum ExprValue {
    Flat(Rc<String>),
    ByRef(RefType, Rc<String>),
    Comparison(Rc<String>, CmpType, Rc<String>),
    Not(Rc<String>),
}

impl From<String> for ExprValue {
    fn from(other: String) -> Self {
        Self::Flat(other.into())
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    precedence: Precedence,
    value: ExprValue,
}

impl Expr {
    pub fn new_atom(expr: &str) -> Self {
        Self {
            precedence: Precedence::Suffix,
            value: ExprValue::Flat(expr.to_owned().into()),
        }
    }
    pub fn new_unary(expr: &str) -> Self {
        Self {
            precedence: Precedence::Unary,
            value: ExprValue::Flat(expr.to_owned().into()),
        }
    }
    pub fn new_str(s: &str) -> Self {
        Self {
            precedence: Precedence::Term,
            value: ExprValue::Flat(format!("{:?}", s).into()),
        }
    }
    pub fn brace(&self) -> Self {
        Expr {
            precedence: Precedence::Term,
            value: match &self.value {
                ExprValue::ByRef(t, v) => ExprValue::ByRef(*t, format!("({})", v).into()),
                _ => ExprValue::Flat(format!("({})", self).into()),
            },
        }
    }
    fn tighten(&self, precedence: Precedence) -> Self {
        if self.precedence < precedence {
            self.brace()
        } else {
            self.clone()
        }
    }
    fn tighten_above(&self, precedence: Precedence) -> Self {
        if self.precedence <= precedence {
            self.brace()
        } else {
            self.clone()
        }
    }
    fn flatten(&self) -> (Precedence, Rc<String>) {
        match &self.value {
            ExprValue::ByRef(_, _) => (
                Precedence::Unary,
                match self.tighten(Precedence::Unary).value {
                    ExprValue::ByRef(RefType::Ref, v) => format!("&{}", v).into(),
                    ExprValue::ByRef(RefType::Mut, v) => format!("&mut {}", v).into(),
                    _ => unreachable!(),
                },
            ),
            ExprValue::Comparison(a, op, b) => {
                (Precedence::Comparison, format!("{} {} {}", a, op, b).into())
            }
            ExprValue::Not(e) => (Precedence::Unary, format!("!{}", e).into()),
            ExprValue::Flat(s) => (self.precedence, s.clone()),
        }
    }
    pub fn by_ref(&self) -> Self {
        let (precedence, s) = self.flatten();
        Expr {
            precedence,
            value: ExprValue::ByRef(RefType::Ref, s),
        }
    }
    pub fn by_mut(&self) -> Self {
        let (precedence, s) = self.flatten();
        Expr {
            precedence,
            value: ExprValue::ByRef(RefType::Mut, s),
        }
    }
    fn auto_deref(&self) -> Self {
        if let ExprValue::ByRef(_, s) = &self.value {
            Expr {
                precedence: self.precedence,
                value: ExprValue::Flat(s.clone()),
            }
        } else {
            self.clone()
        }
    }
    pub fn field(&self, field: &str) -> Self {
        Expr {
            precedence: Precedence::Suffix,
            value: format!(
                "{}.{}",
                self.tighten(Precedence::Suffix).auto_deref(),
                field
            )
            .into(),
        }
    }
    pub fn method<T: fmt::Display>(&self, method: &str, args: T) -> Self {
        Expr {
            precedence: Precedence::Suffix,
            value: format!(
                "{}.{}({})",
                self.tighten(Precedence::Suffix).auto_deref(),
                method,
                args
            )
            .into(),
        }
    }
    pub fn try_(&self) -> Self {
        Expr {
            precedence: Precedence::Suffix,
            value: format!("{}?", self.tighten(Precedence::Suffix)).into(),
        }
    }
    pub fn deref(&self) -> Self {
        if let ExprValue::ByRef(_, s) = &self.value {
            Expr {
                precedence: self.precedence,
                value: ExprValue::Flat(s.clone()),
            }
        } else {
            self.unary("*")
        }
    }
    pub fn unary(&self, op: &str) -> Self {
        Expr {
            precedence: Precedence::Unary,
            value: format!("{}{}", op, self.tighten(Precedence::Unary)).into(),
        }
    }
    pub fn not(&self) -> Self {
        match &self.value {
            ExprValue::Comparison(a, op, b) => Expr {
                precedence: self.precedence,
                value: ExprValue::Comparison(a.clone(), op.inverse(), b.clone()),
            },
            ExprValue::Not(e) => Expr {
                precedence: self.precedence,
                value: ExprValue::Flat(e.clone()),
            },
            _ => Expr {
                precedence: self.precedence,
                value: ExprValue::Not(self.tighten(Precedence::Unary).to_string().into()),
            },
        }
    }
    pub fn compare(&self, op: CmpType, other: &Expr) -> Self {
        let a = self.tighten_above(Precedence::Comparison).to_string();
        let b = other.tighten_above(Precedence::Comparison).to_string();

        Expr {
            precedence: Precedence::Comparison,
            value: ExprValue::Comparison(a.into(), op, b.into()),
        }
    }
    pub fn cast_as(&self, t: &str) -> Self {
        Expr {
            precedence: Precedence::Cast,
            value: format!("{} as {}", self.tighten(Precedence::Cast), t).into(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.flatten().1.fmt(f)
    }
}
