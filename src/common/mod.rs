pub mod module;
pub mod program {
    use super::*;
    use std::collections::HashMap;
    pub type Program = HashMap<String, module::Module>;
}
