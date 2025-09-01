use jetcrab::vm::types::names::*;
use serde_json;

#[cfg(test)]
mod tests {
    use super::*;

    // VariableName tests
    #[test]
    fn test_variable_name_new() {
        let name = VariableName::new("test_var".to_string());
        assert_eq!(name.as_str(), "test_var");
    }

    #[test]
    fn test_variable_name_as_str() {
        let name = VariableName::new("myVariable".to_string());
        assert_eq!(name.as_str(), "myVariable");
    }

    #[test]
    fn test_variable_name_as_string() {
        let name = VariableName::new("test".to_string());
        let string = name.as_string();
        assert_eq!(string, "test");
    }

    #[test]
    fn test_variable_name_is_empty() {
        let empty_name = VariableName::new("".to_string());
        assert!(empty_name.is_empty());

        let non_empty_name = VariableName::new("test".to_string());
        assert!(!non_empty_name.is_empty());
    }

    #[test]
    fn test_variable_name_len() {
        let name = VariableName::new("hello".to_string());
        assert_eq!(name.len(), 5);

        let empty_name = VariableName::new("".to_string());
        assert_eq!(empty_name.len(), 0);
    }

    #[test]
    fn test_variable_name_from_string() {
        let name: VariableName = "test_var".to_string().into();
        assert_eq!(name.as_str(), "test_var");
    }

    #[test]
    fn test_variable_name_from_str() {
        let name: VariableName = "test_var".into();
        assert_eq!(name.as_str(), "test_var");
    }

    #[test]
    fn test_variable_name_into_string() {
        let name = VariableName::new("test_var".to_string());
        let string: String = name.into();
        assert_eq!(string, "test_var");
    }

    #[test]
    fn test_variable_name_display() {
        let name = VariableName::new("test_var".to_string());
        assert_eq!(format!("{}", name), "test_var");
    }

    #[test]
    fn test_variable_name_as_ref() {
        let name = VariableName::new("test_var".to_string());
        let s: &str = name.as_ref();
        assert_eq!(s, "test_var");
    }

    #[test]
    fn test_variable_name_deref() {
        let name = VariableName::new("test_var".to_string());
        assert_eq!(&*name, "test_var");
    }

    #[test]
    fn test_variable_name_clone() {
        let name1 = VariableName::new("test_var".to_string());
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_variable_name_partial_eq() {
        let name1 = VariableName::new("test_var".to_string());
        let name2 = VariableName::new("test_var".to_string());
        let name3 = VariableName::new("different".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_variable_name_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let name1 = VariableName::new("test_var".to_string());
        let name2 = VariableName::new("test_var".to_string());

        map.insert(name1, "value1");
        assert_eq!(map.get(&name2), Some(&"value1"));
    }

    #[test]
    fn test_variable_name_serialization() {
        let name = VariableName::new("test_var".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"test_var\"");
    }

    #[test]
    fn test_variable_name_deserialization() {
        let json = "\"test_var\"";
        let name: VariableName = serde_json::from_str(json).unwrap();
        assert_eq!(name.as_str(), "test_var");
    }

    // FunctionName tests
    #[test]
    fn test_function_name_new() {
        let name = FunctionName::new("testFunction".to_string());
        assert_eq!(name.as_str(), "testFunction");
    }

    #[test]
    fn test_function_name_as_str() {
        let name = FunctionName::new("myFunction".to_string());
        assert_eq!(name.as_str(), "myFunction");
    }

    #[test]
    fn test_function_name_as_string() {
        let name = FunctionName::new("test".to_string());
        let string = name.as_string();
        assert_eq!(string, "test");
    }

    #[test]
    fn test_function_name_is_empty() {
        let empty_name = FunctionName::new("".to_string());
        assert!(empty_name.is_empty());

        let non_empty_name = FunctionName::new("test".to_string());
        assert!(!non_empty_name.is_empty());
    }

    #[test]
    fn test_function_name_len() {
        let name = FunctionName::new("hello".to_string());
        assert_eq!(name.len(), 5);

        let empty_name = FunctionName::new("".to_string());
        assert_eq!(empty_name.len(), 0);
    }

    #[test]
    fn test_function_name_from_string() {
        let name: FunctionName = "testFunction".to_string().into();
        assert_eq!(name.as_str(), "testFunction");
    }

    #[test]
    fn test_function_name_from_str() {
        let name: FunctionName = "testFunction".into();
        assert_eq!(name.as_str(), "testFunction");
    }

    #[test]
    fn test_function_name_into_string() {
        let name = FunctionName::new("testFunction".to_string());
        let string: String = name.into();
        assert_eq!(string, "testFunction");
    }

    #[test]
    fn test_function_name_display() {
        let name = FunctionName::new("testFunction".to_string());
        assert_eq!(format!("{}", name), "testFunction");
    }

    #[test]
    fn test_function_name_as_ref() {
        let name = FunctionName::new("testFunction".to_string());
        let s: &str = name.as_ref();
        assert_eq!(s, "testFunction");
    }

    #[test]
    fn test_function_name_deref() {
        let name = FunctionName::new("testFunction".to_string());
        assert_eq!(&*name, "testFunction");
    }

    #[test]
    fn test_function_name_clone() {
        let name1 = FunctionName::new("testFunction".to_string());
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_function_name_partial_eq() {
        let name1 = FunctionName::new("testFunction".to_string());
        let name2 = FunctionName::new("testFunction".to_string());
        let name3 = FunctionName::new("different".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_function_name_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let name1 = FunctionName::new("testFunction".to_string());
        let name2 = FunctionName::new("testFunction".to_string());

        map.insert(name1, "value1");
        assert_eq!(map.get(&name2), Some(&"value1"));
    }

    #[test]
    fn test_function_name_serialization() {
        let name = FunctionName::new("testFunction".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"testFunction\"");
    }

    #[test]
    fn test_function_name_deserialization() {
        let json = "\"testFunction\"";
        let name: FunctionName = serde_json::from_str(json).unwrap();
        assert_eq!(name.as_str(), "testFunction");
    }

    // ClassName tests
    #[test]
    fn test_class_name_new() {
        let name = ClassName::new("TestClass".to_string());
        assert_eq!(name.as_str(), "TestClass");
    }

    #[test]
    fn test_class_name_as_str() {
        let name = ClassName::new("MyClass".to_string());
        assert_eq!(name.as_str(), "MyClass");
    }

    #[test]
    fn test_class_name_as_string() {
        let name = ClassName::new("test".to_string());
        let string = name.as_string();
        assert_eq!(string, "test");
    }

    #[test]
    fn test_class_name_is_empty() {
        let empty_name = ClassName::new("".to_string());
        assert!(empty_name.is_empty());

        let non_empty_name = ClassName::new("test".to_string());
        assert!(!non_empty_name.is_empty());
    }

    #[test]
    fn test_class_name_len() {
        let name = ClassName::new("hello".to_string());
        assert_eq!(name.len(), 5);

        let empty_name = ClassName::new("".to_string());
        assert_eq!(empty_name.len(), 0);
    }

    #[test]
    fn test_class_name_from_string() {
        let name: ClassName = "TestClass".to_string().into();
        assert_eq!(name.as_str(), "TestClass");
    }

    #[test]
    fn test_class_name_from_str() {
        let name: ClassName = "TestClass".into();
        assert_eq!(name.as_str(), "TestClass");
    }

    #[test]
    fn test_class_name_into_string() {
        let name = ClassName::new("TestClass".to_string());
        let string: String = name.into();
        assert_eq!(string, "TestClass");
    }

    #[test]
    fn test_class_name_display() {
        let name = ClassName::new("TestClass".to_string());
        assert_eq!(format!("{}", name), "TestClass");
    }

    #[test]
    fn test_class_name_as_ref() {
        let name = ClassName::new("TestClass".to_string());
        let s: &str = name.as_ref();
        assert_eq!(s, "TestClass");
    }

    #[test]
    fn test_class_name_deref() {
        let name = ClassName::new("TestClass".to_string());
        assert_eq!(&*name, "TestClass");
    }

    #[test]
    fn test_class_name_clone() {
        let name1 = ClassName::new("TestClass".to_string());
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_class_name_partial_eq() {
        let name1 = ClassName::new("TestClass".to_string());
        let name2 = ClassName::new("TestClass".to_string());
        let name3 = ClassName::new("different".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_class_name_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let name1 = ClassName::new("TestClass".to_string());
        let name2 = ClassName::new("TestClass".to_string());

        map.insert(name1, "value1");
        assert_eq!(map.get(&name2), Some(&"value1"));
    }

    #[test]
    fn test_class_name_serialization() {
        let name = ClassName::new("TestClass".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"TestClass\"");
    }

    #[test]
    fn test_class_name_deserialization() {
        let json = "\"TestClass\"";
        let name: ClassName = serde_json::from_str(json).unwrap();
        assert_eq!(name.as_str(), "TestClass");
    }

    // PropertyName tests
    #[test]
    fn test_property_name_new() {
        let name = PropertyName::new("testProperty".to_string());
        assert_eq!(name.as_str(), "testProperty");
    }

    #[test]
    fn test_property_name_as_str() {
        let name = PropertyName::new("myProperty".to_string());
        assert_eq!(name.as_str(), "myProperty");
    }

    #[test]
    fn test_property_name_as_string() {
        let name = PropertyName::new("test".to_string());
        let string = name.as_string();
        assert_eq!(string, "test");
    }

    #[test]
    fn test_property_name_is_empty() {
        let empty_name = PropertyName::new("".to_string());
        assert!(empty_name.is_empty());

        let non_empty_name = PropertyName::new("test".to_string());
        assert!(!non_empty_name.is_empty());
    }

    #[test]
    fn test_property_name_len() {
        let name = PropertyName::new("hello".to_string());
        assert_eq!(name.len(), 5);

        let empty_name = PropertyName::new("".to_string());
        assert_eq!(empty_name.len(), 0);
    }

    #[test]
    fn test_property_name_from_string() {
        let name: PropertyName = "testProperty".to_string().into();
        assert_eq!(name.as_str(), "testProperty");
    }

    #[test]
    fn test_property_name_from_str() {
        let name: PropertyName = "testProperty".into();
        assert_eq!(name.as_str(), "testProperty");
    }

    #[test]
    fn test_property_name_into_string() {
        let name = PropertyName::new("testProperty".to_string());
        let string: String = name.into();
        assert_eq!(string, "testProperty");
    }

    #[test]
    fn test_property_name_display() {
        let name = PropertyName::new("testProperty".to_string());
        assert_eq!(format!("{}", name), "testProperty");
    }

    #[test]
    fn test_property_name_as_ref() {
        let name = PropertyName::new("testProperty".to_string());
        let s: &str = name.as_ref();
        assert_eq!(s, "testProperty");
    }

    #[test]
    fn test_property_name_deref() {
        let name = PropertyName::new("testProperty".to_string());
        assert_eq!(&*name, "testProperty");
    }

    #[test]
    fn test_property_name_clone() {
        let name1 = PropertyName::new("testProperty".to_string());
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_property_name_partial_eq() {
        let name1 = PropertyName::new("testProperty".to_string());
        let name2 = PropertyName::new("testProperty".to_string());
        let name3 = PropertyName::new("different".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_property_name_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let name1 = PropertyName::new("testProperty".to_string());
        let name2 = PropertyName::new("testProperty".to_string());

        map.insert(name1, "value1");
        assert_eq!(map.get(&name2), Some(&"value1"));
    }

    #[test]
    fn test_property_name_serialization() {
        let name = PropertyName::new("testProperty".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"testProperty\"");
    }

    #[test]
    fn test_property_name_deserialization() {
        let json = "\"testProperty\"";
        let name: PropertyName = serde_json::from_str(json).unwrap();
        assert_eq!(name.as_str(), "testProperty");
    }

    // ModuleName tests
    #[test]
    fn test_module_name_new() {
        let name = ModuleName::new("testModule".to_string());
        assert_eq!(name.as_str(), "testModule");
    }

    #[test]
    fn test_module_name_as_str() {
        let name = ModuleName::new("myModule".to_string());
        assert_eq!(name.as_str(), "myModule");
    }

    #[test]
    fn test_module_name_as_string() {
        let name = ModuleName::new("test".to_string());
        let string = name.as_string();
        assert_eq!(string, "test");
    }

    #[test]
    fn test_module_name_is_empty() {
        let empty_name = ModuleName::new("".to_string());
        assert!(empty_name.is_empty());

        let non_empty_name = ModuleName::new("test".to_string());
        assert!(!non_empty_name.is_empty());
    }

    #[test]
    fn test_module_name_len() {
        let name = ModuleName::new("hello".to_string());
        assert_eq!(name.len(), 5);

        let empty_name = ModuleName::new("".to_string());
        assert_eq!(empty_name.len(), 0);
    }

    #[test]
    fn test_module_name_from_string() {
        let name: ModuleName = "testModule".to_string().into();
        assert_eq!(name.as_str(), "testModule");
    }

    #[test]
    fn test_module_name_from_str() {
        let name: ModuleName = "testModule".into();
        assert_eq!(name.as_str(), "testModule");
    }

    #[test]
    fn test_module_name_into_string() {
        let name = ModuleName::new("testModule".to_string());
        let string: String = name.into();
        assert_eq!(string, "testModule");
    }

    #[test]
    fn test_module_name_display() {
        let name = ModuleName::new("testModule".to_string());
        assert_eq!(format!("{}", name), "testModule");
    }

    #[test]
    fn test_module_name_as_ref() {
        let name = ModuleName::new("testModule".to_string());
        let s: &str = name.as_ref();
        assert_eq!(s, "testModule");
    }

    #[test]
    fn test_module_name_deref() {
        let name = ModuleName::new("testModule".to_string());
        assert_eq!(&*name, "testModule");
    }

    #[test]
    fn test_module_name_clone() {
        let name1 = ModuleName::new("testModule".to_string());
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_module_name_partial_eq() {
        let name1 = ModuleName::new("testModule".to_string());
        let name2 = ModuleName::new("testModule".to_string());
        let name3 = ModuleName::new("different".to_string());

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_module_name_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let name1 = ModuleName::new("testModule".to_string());
        let name2 = ModuleName::new("testModule".to_string());

        map.insert(name1, "value1");
        assert_eq!(map.get(&name2), Some(&"value1"));
    }

    #[test]
    fn test_module_name_serialization() {
        let name = ModuleName::new("testModule".to_string());
        let serialized = serde_json::to_string(&name).unwrap();
        assert_eq!(serialized, "\"testModule\"");
    }

    #[test]
    fn test_module_name_deserialization() {
        let json = "\"testModule\"";
        let name: ModuleName = serde_json::from_str(json).unwrap();
        assert_eq!(name.as_str(), "testModule");
    }

    // Cross-type tests
    #[test]
    fn test_different_name_types_are_not_equal() {
        let var_name = VariableName::new("test".to_string());
        let func_name = FunctionName::new("test".to_string());
        let class_name = ClassName::new("test".to_string());
        let prop_name = PropertyName::new("test".to_string());
        let mod_name = ModuleName::new("test".to_string());

        // They should have the same string content but be different types
        assert_eq!(var_name.as_str(), "test");
        assert_eq!(func_name.as_str(), "test");
        assert_eq!(class_name.as_str(), "test");
        assert_eq!(prop_name.as_str(), "test");
        assert_eq!(mod_name.as_str(), "test");

        // The types themselves are different, so we can't compare them directly
        // This test just verifies they all work correctly with the same string content
    }

    #[test]
    fn test_name_types_with_special_characters() {
        let special_chars = "test_name_with_underscores_and123numbers";

        let var_name = VariableName::new(special_chars.to_string());
        let func_name = FunctionName::new(special_chars.to_string());
        let class_name = ClassName::new(special_chars.to_string());
        let prop_name = PropertyName::new(special_chars.to_string());
        let mod_name = ModuleName::new(special_chars.to_string());

        assert_eq!(var_name.as_str(), special_chars);
        assert_eq!(func_name.as_str(), special_chars);
        assert_eq!(class_name.as_str(), special_chars);
        assert_eq!(prop_name.as_str(), special_chars);
        assert_eq!(mod_name.as_str(), special_chars);
    }

    #[test]
    fn test_name_types_with_unicode() {
        let unicode_name = "测试名称_αβγ_🚀";

        let var_name = VariableName::new(unicode_name.to_string());
        let func_name = FunctionName::new(unicode_name.to_string());
        let class_name = ClassName::new(unicode_name.to_string());
        let prop_name = PropertyName::new(unicode_name.to_string());
        let mod_name = ModuleName::new(unicode_name.to_string());

        assert_eq!(var_name.as_str(), unicode_name);
        assert_eq!(func_name.as_str(), unicode_name);
        assert_eq!(class_name.as_str(), unicode_name);
        assert_eq!(prop_name.as_str(), unicode_name);
        assert_eq!(mod_name.as_str(), unicode_name);
    }

    #[test]
    fn test_name_types_debug_formatting() {
        let var_name = VariableName::new("test_var".to_string());
        let func_name = FunctionName::new("test_func".to_string());
        let class_name = ClassName::new("TestClass".to_string());
        let prop_name = PropertyName::new("test_prop".to_string());
        let mod_name = ModuleName::new("test_mod".to_string());

        // Test that debug formatting works
        let var_debug = format!("{:?}", var_name);
        let func_debug = format!("{:?}", func_name);
        let class_debug = format!("{:?}", class_name);
        let prop_debug = format!("{:?}", prop_name);
        let mod_debug = format!("{:?}", mod_name);

        assert!(var_debug.contains("test_var"));
        assert!(func_debug.contains("test_func"));
        assert!(class_debug.contains("TestClass"));
        assert!(prop_debug.contains("test_prop"));
        assert!(mod_debug.contains("test_mod"));
    }

    #[test]
    fn test_name_types_serialization_roundtrip() {
        let original_var = VariableName::new("test_var".to_string());
        let original_func = FunctionName::new("test_func".to_string());
        let original_class = ClassName::new("TestClass".to_string());
        let original_prop = PropertyName::new("test_prop".to_string());
        let original_mod = ModuleName::new("test_mod".to_string());

        // Serialize and deserialize
        let var_json = serde_json::to_string(&original_var).unwrap();
        let func_json = serde_json::to_string(&original_func).unwrap();
        let class_json = serde_json::to_string(&original_class).unwrap();
        let prop_json = serde_json::to_string(&original_prop).unwrap();
        let mod_json = serde_json::to_string(&original_mod).unwrap();

        let deserialized_var: VariableName = serde_json::from_str(&var_json).unwrap();
        let deserialized_func: FunctionName = serde_json::from_str(&func_json).unwrap();
        let deserialized_class: ClassName = serde_json::from_str(&class_json).unwrap();
        let deserialized_prop: PropertyName = serde_json::from_str(&prop_json).unwrap();
        let deserialized_mod: ModuleName = serde_json::from_str(&mod_json).unwrap();

        // Should be equal after roundtrip
        assert_eq!(original_var, deserialized_var);
        assert_eq!(original_func, deserialized_func);
        assert_eq!(original_class, deserialized_class);
        assert_eq!(original_prop, deserialized_prop);
        assert_eq!(original_mod, deserialized_mod);
    }
}
