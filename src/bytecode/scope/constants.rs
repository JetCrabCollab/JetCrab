use crate::vm::types::ConstantIndex;
use std::collections::HashMap;

pub trait ConstantManager {
    fn add_constant(&mut self, value: String) -> ConstantIndex;
    fn get_constants(&self) -> &Vec<String>;
}

pub trait ConstantCore {
    fn constants(&self) -> &Vec<String>;
    fn constant_map(&self) -> &HashMap<String, ConstantIndex>;
    fn constants_mut(&mut self) -> &mut Vec<String>;
    fn constant_map_mut(&mut self) -> &mut HashMap<String, ConstantIndex>;
}

impl<T> ConstantManager for T
where
    T: ConstantCore,
{
    fn add_constant(&mut self, value: String) -> ConstantIndex {
        if let Some(&id) = self.constant_map().get(&value) {
            id
        } else {
            let id = ConstantIndex::new(self.constants().len());
            self.constants_mut().push(value.clone());
            self.constant_map_mut().insert(value, id);
            id
        }
    }

    fn get_constants(&self) -> &Vec<String> {
        self.constants()
    }
}
