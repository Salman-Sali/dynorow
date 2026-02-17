use field_info::FieldInfo;
use key::Key;
use regex::Regex;
use syn::{Data, DeriveInput, Fields};

pub mod field_info;
pub mod field_type;
pub mod key;

pub struct StructInfo {
    pub struct_name: String,
    pub pk: Option<String>,
    pub pk_value: Option<String>,
    pub pk_value_parts: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub table_name_provider: Option<String>,
    pub dynorow: bool,
}

impl StructInfo {
    pub fn new(input: DeriveInput, dynorow: bool) -> Self {
        let mut struct_info = Self {
            struct_name: input.ident.to_string(),
            pk: None,
            pk_value: None,
            fields: vec![],
            table_name_provider: None,
            dynorow,
            pk_value_parts: vec![],
        };

        if struct_info.dynorow {
            for attribute in input.attrs {
                if !attribute.path().is_ident("dynorow") {
                    continue;
                }

                let _ = attribute.parse_nested_meta(|meta| {
                    let Some(ident) = meta.path.get_ident() else {
                        return Ok(());
                    };
                    match ident.to_string().as_str() {
                        "pk" => {
                            let Ok(key) = meta.value() else {
                                panic!("Error while getting pk key for struct.");
                            };
                            struct_info.set_pk(key.to_string());
                        }
                        "pk_value" => {
                            let Ok(value) = meta.value() else {
                                panic!("Error while getting pk key value for struct.");
                            };
                            struct_info.set_pk_value(value.to_string());
                        }
                        "table" => {
                            let Ok(value) = meta.value() else {
                                panic!("Error while getting pk key value for struct.");
                            };
                            struct_info.table_name_provider = Some(value.to_string());
                        }
                        _ => {}
                    }
                    return Ok(());
                });
            }
        }

        let fields = if let Data::Struct(data) = input.data {
            if let Fields::Named(fields) = data.fields {
                fields
            } else {
                panic!("Only named fields are supported.");
            }
        } else {
            panic!("Only structs are supported.");
        };

        for field in fields.named.iter() {
            struct_info.insert_field(FieldInfo::try_from(field).unwrap());
        }

        struct_info.panic_at_errors();

        return struct_info;
    }

    pub fn is_static_pk_value(&self) -> bool {
        self.pk_value.is_some() && self.pk_value_parts.is_empty()
    }

    pub fn is_generated_pk_value(&self) -> bool {
        self.pk_value.is_some() && !self.pk_value_parts.is_empty()
    }

    pub fn set_pk(&mut self, pk: String) {
        self.pk = Some(pk.replace("\"", ""));
    }

    pub fn set_pk_value(&mut self, mut pk_value: String) {
        pk_value = pk_value.replace("\"", "");
        let regex = Regex::new(r"\{([^}]*)\}").unwrap();

        let pk_value_clone = pk_value.clone();
        let finds: Vec<_> = regex.find_iter(&pk_value_clone).collect();

        let mut arguments = vec![];

        for find in finds {
            let argument = find
                .as_str()
                .trim_start_matches("{")
                .trim_end_matches("}")
                .to_string();
            pk_value = pk_value.replace(&argument, "");
            arguments.push(argument);
        }
        self.pk_value_parts = arguments;
        self.pk_value = Some(pk_value);
    }

    pub fn find_in_handled_fields(&self, field_name: &str) -> Option<&FieldInfo> {
        self.fields
            .iter()
            .find(|x| !x.ignore && x.name == field_name)
    }

    pub fn get_handled_fields(&self) -> Vec<&FieldInfo> {
        self.fields.iter().filter(|x| !x.ignore).collect()
    }

    fn insert_field(&mut self, field: FieldInfo) {
        if !field.ignore {
            let key = field.get_key_str();
            let key_already_exists = self
                .get_handled_fields()
                .iter()
                .any(|x| x.get_key_str() == key);
            if key_already_exists {
                panic!("Duplicate key : {}", key)
            }
        }
        self.fields.push(field);
    }

    pub fn get_pk_key(&self) -> String {
        let Some(pk) = &self.pk else {
            let Some(pk_field) = self.fields.iter().find(|x| matches!(x.key, Key::Pk(_))) else {
                panic!("Cannot find pk key");
            };
            return pk_field.get_key_str();
        };
        return pk.clone();
    }

    pub fn get_pk_field(&self) -> Option<&FieldInfo> {
        self.fields.iter().find(|x| matches!(x.key, Key::Pk(_)))
    }

    pub fn get_sk_field(&self) -> Option<&FieldInfo> {
        self.fields.iter().find(|x| matches!(x.key, Key::Sk(_)))
    }

    pub fn get_sk_key(&self) -> Option<String> {
        let Some(sk_field) = self.fields.iter().find(|x| matches!(x.key, Key::Sk(_))) else {
            return None;
        };
        Some(sk_field.get_key_str())
    }

    pub fn struct_has_pk(&self) -> bool {
        self.pk.is_some()
    }

    pub fn struct_has_pk_value(&self) -> bool {
        self.pk_value.is_some()
    }

    pub fn generate_projection_expression(&self) -> String {
        self.get_handled_fields()
            .iter()
            .map(|x| x.as_projection_variable())
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn panic_at_errors(&self) {
        if !self.dynorow {
            return;
        }

        let struct_has_pk = self.struct_has_pk();
        let struct_has_pk_value = self.struct_has_pk_value();
        let pk_field = self.fields.iter().find(|x| matches!(x.key, Key::Pk(_)));
        let field_has_pk = pk_field.is_some();

        let sk_field = self.fields.iter().find(|x| matches!(x.key, Key::Sk(_)));
        let field_has_sk = sk_field.is_some();
        if field_has_sk && sk_field.unwrap().is_option {
            panic!("SK field cannot be of type Option<T>");
        }

        if field_has_pk && pk_field.unwrap().is_option {
            panic!("Pk field cannot be of type Option<T>");
        }

        if struct_has_pk && !struct_has_pk_value {
            panic!("Provide pk value when defining pk at struct level.");
        }

        if !struct_has_pk && struct_has_pk_value {
            panic!("Provide pk key when providing pk value at struct level.");
        }

        if (struct_has_pk && struct_has_pk_value) && field_has_pk {
            panic!("Found pk info at both struct and field level. Only one is required.")
        }

        if !(struct_has_pk || struct_has_pk_value) && !field_has_pk {
            panic!("No pk info found.")
        }
    }
}
