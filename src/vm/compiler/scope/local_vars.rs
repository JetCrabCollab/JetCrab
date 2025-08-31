use crate::vm::types::LocalIndex;
use std::collections::HashMap;

pub trait ScopeManager {
    fn get_or_create_local(&mut self, name: &str) -> LocalIndex;
    fn get_local(&self, name: &str) -> Option<&LocalIndex>;
    fn is_array_variable(&self, name: &str) -> bool;
}

pub trait ScopeCore {
    fn local_vars(&self) -> &HashMap<String, LocalIndex>;
    fn local_vars_mut(&mut self) -> &mut HashMap<String, LocalIndex>;
    fn next_local(&self) -> usize;
    fn set_next_local(&mut self, next: usize);
}

impl<T> ScopeManager for T
where
    T: ScopeCore,
{
    fn get_or_create_local(&mut self, name: &str) -> LocalIndex {
        if let Some(&idx) = self.local_vars().get(name) {
            idx
        } else {
            let idx = LocalIndex::new(self.next_local());
            self.local_vars_mut().insert(name.to_string(), idx);
            self.set_next_local(self.next_local() + 1);
            idx
        }
    }

    fn get_local(&self, name: &str) -> Option<&LocalIndex> {
        self.local_vars().get(name)
    }

    fn is_array_variable(&self, name: &str) -> bool {
        // Simple heuristic: check if variable name suggests it's an array
        let name_lower = name.to_lowercase();
        name_lower.contains("array")
            || name_lower.contains("list")
            || name_lower.contains("items")
            || name_lower.contains("fruits")
            || name_lower.contains("numbers")
            || name_lower.contains("colors")
            || name_lower.contains("names")
            || name_lower.contains("data")
            || name_lower.contains("elements")
            || name_lower.contains("values")
            || name_lower.contains("collection")
            || name_lower.contains("set")
            || name_lower.contains("group")
            || name_lower.contains("bunch")
            || name_lower.contains("lot")
            || name_lower.contains("series")
            || name_lower.contains("sequence")
            || name_lower.contains("row")
            || name_lower.contains("column")
            || name_lower.contains("stack")
            || name_lower.contains("queue")
            || name_lower.contains("heap")
            || name_lower.contains("tree")
            || name_lower.contains("graph")
            || name_lower.contains("matrix")
            || name_lower.contains("table")
            || name_lower.contains("grid")
            || name_lower.contains("board")
            || name_lower.contains("deck")
            || name_lower.contains("pack")
            || name_lower.contains("bundle")
            || name_lower.contains("batch")
            || name_lower.contains("cluster")
            || name_lower.contains("arr")
    }
}
