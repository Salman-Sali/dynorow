use crate::key::Key;

pub trait HasKey {
    fn get_key() -> Key;
}