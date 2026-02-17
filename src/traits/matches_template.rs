use crate::traits::has_pk_value_template::HasPkValueTemplate;

pub trait MatchesTemplate {
    fn matches_template(value: &str) -> bool;
}

impl<T> MatchesTemplate for T
where
    T: HasPkValueTemplate,
{
    fn matches_template(value: &str) -> bool {
        let mut value = value;
        let template = T::get_pk_value_template();
        let parts = template.split("{}").collect::<Vec<&str>>();
        if parts.is_empty() {
            return false;
        }

        let begins_with_variable = template.starts_with("{}");
        if !begins_with_variable {
            if !value.starts_with(parts[0]) {
                return false;
            }
        }

        let ends_with_variable = template.ends_with("{}");
        if !ends_with_variable {
            if !value.ends_with(parts[parts.len() - 1]) {
                return false;
            }
        }

        for part in parts {
            if let Some((_, rest)) = value.split_once(part) {
                value = rest;
            } else {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
pub mod test {
    use crate::{self as dynorow, traits::matches_template::MatchesTemplate};
    use dynorow_derive::DynoRow;

    #[derive(Debug, Clone, DynoRow)]
    #[dynorow(pk = "pk")]
    #[dynorow(pk_value = "Order:{user_id}:{order_id}")]
    pub struct OrderPayment {
        #[dynorow(sk = "sk")]
        pub payment_id: String,
        pub order_id: String,
        pub user_id: String,
    }

    #[test]
    pub fn test_tempalte_matches() {
        assert!(OrderPayment::matches_template(
            "Order:user_123123:order_123234"
        ));

        assert!(!OrderPayment::matches_template("Order:order_123"));

        assert!(!OrderPayment::matches_template(
            "PaymentStatus:user_1234:payment_12312"
        ));
    }
}
