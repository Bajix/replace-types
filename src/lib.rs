use std::collections::HashMap;

pub use syn::{visit_mut::VisitMut, TypePath};

/// A [`syn`] [visitor](syn::visit_mut::VisitMut) that replaces types
pub struct ReplaceTypes(HashMap<TypePath, TypePath>);

impl ReplaceTypes {
    pub fn new(substitutions: HashMap<TypePath, TypePath>) -> Self {
        ReplaceTypes(substitutions)
    }
}

impl VisitMut for ReplaceTypes {
    fn visit_type_path_mut(&mut self, node: &mut TypePath) {
        if let Some(substitution) = self.0.get(node) {
            *node = substitution.to_owned();
        }
    }
}
