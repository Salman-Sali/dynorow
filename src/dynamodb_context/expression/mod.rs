pub mod conditional;
pub mod update;

pub struct ExpressionContext {
    pub val_count: u32,
    pub prefix: String,
}

impl ExpressionContext {
    pub fn new(prefix: &str) -> Self {
        Self {
            val_count: 0,
            prefix: prefix.to_string(),
        }
    }

    pub fn next(&mut self) -> String {
        self.val_count += 1;
        format!(":{}{}", self.prefix, self.val_count)
    }
}

pub trait AsVariable {
    fn as_variable(&self) -> String;
}

impl AsVariable for &str {
    fn as_variable(&self) -> String {
        format!("#var_{}", self.replace('.', "_"))
    }
}

impl AsVariable for String {
    fn as_variable(&self) -> String {
        format!("#var_{}", self.replace('.', "_"))
    }
}
