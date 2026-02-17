use crate::traits::has_pk_value::HasStaticPkValue;

pub trait PkEquals {
    fn pk_equals(pk: &str) -> bool;
}

impl<T> PkEquals for T
where
    T: HasStaticPkValue,
{
    fn pk_equals(pk: &str) -> bool {
        T::get_static_pk_value() == pk
    }
}
