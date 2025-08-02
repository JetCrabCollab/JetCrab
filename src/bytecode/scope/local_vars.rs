use crate::vm::types::LocalIndex;
use std::collections::HashMap;

pub trait ScopeManager {
    fn get_or_create_local(&mut self, name: &str) -> LocalIndex;
    fn get_local(&self, name: &str) -> Option<&LocalIndex>;
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
}
