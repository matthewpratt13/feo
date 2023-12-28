#[derive(Debug, Clone)]
pub enum WarningKind {}

#[derive(Debug, Clone)]
pub struct CompilerWarning {
    pub warning_kind: WarningKind,
    pub pos: usize,
}
