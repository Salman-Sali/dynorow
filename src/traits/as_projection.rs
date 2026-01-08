use std::collections::HashMap;

pub trait AsProjection {
    fn as_projection() -> String;
    fn as_projection_names() -> HashMap<String, String>;
}