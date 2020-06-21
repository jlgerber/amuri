use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STMAP: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        m.insert("maya_model".to_string(), "mb");
        m.insert("alembic_model".to_string(), "abc");
        m.insert("alembic_cache".to_string(), "abc");
        m.insert("usd_model".to_string(), "usd");
        m.insert("usd_scene".to_string(), "usda");
        m
    };
    
}
