use enum_newtype_macro::enum_newtype;

#[enum_newtype(name = OpVariants, aliases = true)]
#[derive(Debug, Copy, Clone)]
pub enum Op {
    /// Add docs.
    Add { lhs: Reg, rhs: Reg },
    /// Sub docs.
    Sub { lhs: Reg, rhs: Reg },
    /// Mul docs.
    Mul { lhs: Reg, rhs: Reg },
    /// Const docs.
    Const { value: Value },
}

impl Op {
    pub fn eval(&self, regs: &[Value]) -> Value {
        Value(match self {
            Self::Add(op) => regs[op.lhs.0].0 + regs[op.rhs.0].0,
            Self::Sub(op) => regs[op.lhs.0].0 - regs[op.rhs.0].0,
            Self::Mul(op) => regs[op.lhs.0].0 * regs[op.rhs.0].0,
            Self::Const(op) => op.value.0,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Reg(usize);

#[derive(Debug, Copy, Clone)]
pub struct Value(u64);
