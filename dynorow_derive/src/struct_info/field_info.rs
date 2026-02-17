use quote::{ToTokens, quote};
use syn::{Field, PathArguments, PathSegment, Type};

use super::{field_type::FieldType, key::Key};

pub struct FieldInfo {
    pub name: String,
    pub key: Key,
    pub field_type: FieldType,
    pub field_syn_type: Type,
    pub ignore: bool,
    pub is_option: bool,
    pub is_serde: bool,
}

impl FieldInfo {
    pub fn new(
        name: String,
        key: Key,
        field_type: String,
        field_syn_type: Type,
        ignore: bool,
        is_option: bool,
        is_serde: bool,
    ) -> Self {
        Self {
            name,
            key,
            field_type: FieldType::from(field_type),
            field_syn_type,
            ignore,
            is_option,
            is_serde,
        }
    }

    pub fn get_key_str(&self) -> String {
        match &self.key {
            Key::Key(x) => x,
            Key::Pk(x) => x,
            Key::Sk(x) => x,
        }
        .clone()
    }

    pub fn as_projection_variable(&self) -> String {
        format!("#v_{}", self.get_key_str())
    }

    pub fn get_type_str(&self) -> String {
        let field_type_str = self.field_type.to_string();
        match self.is_option {
            true => format!("Option::<{}>", field_type_str),
            false => field_type_str,
        }
    }

    pub fn get_type_token(&self) -> proc_macro2::TokenStream {
        let field_syn_type = self.field_syn_type.clone();
        match self.is_option {
            true => quote! {Option::<#field_syn_type>},
            false => quote! {#field_syn_type},
        }
    }
}

struct FieldScan {
    pub field_name: String,
    pub key: String,
    pub field_type: String,
    pub field_syn_type: Type,
    pub is_pk_key: bool,
    pub is_sk_key: bool,
    pub ignore: bool,
    pub is_option: bool,
    pub is_serde: bool,
}

impl Into<FieldInfo> for FieldScan {
    fn into(self) -> FieldInfo {
        FieldInfo::new(
            self.field_name.clone(),
            self.get_key(),
            self.field_type,
            self.field_syn_type,
            self.ignore,
            self.is_option,
            self.is_serde,
        )
    }
}

fn get_last_path_segment(ty: &Type) -> &PathSegment {
    match ty {
        Type::Path(type_path) => match type_path.path.segments.last() {
            Some(segment) => {
                return segment;
            }
            None => panic!("Error while fetching type name."),
        },
        _ => panic!("Unhandled syn::Type"),
    }
}

fn get_type_ident(ty: &Type) -> String {
    //get_last_path_segment(ty).ident.to_string()
    match ty {
        Type::Path(type_path) => type_path.into_token_stream().to_string(),
        _ => panic!("Unhandled syn::Type"),
    }
}

fn get_generic_type_argument(ty: &Type) -> Type {
    let segement = get_last_path_segment(ty);
    match &segement.arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(generic_argument) => match generic_argument {
                syn::GenericArgument::Type(t) => return t.clone(),
                _ => panic!("Unhandled generic argument."),
            },
            None => panic!("Type does not have any generic argument."),
        },
        _ => panic!("Unhandled path segment argument."),
    }
}

impl FieldScan {
    fn new(field: &Field) -> Self {
        let mut is_option = false;
        let mut field_syn_type = field.ty.clone();
        let mut field_type = get_type_ident(&field.ty);
        if field_type.starts_with("Option") {
            is_option = true;
            let generic_argument_type = get_generic_type_argument(&field.ty);
            field_syn_type = generic_argument_type.clone();
            field_type = get_type_ident(&generic_argument_type);
        }

        let key = field.ident.as_ref().unwrap().to_string().replace("\"", "");

        Self {
            field_name: key.clone(),
            key,
            field_type,
            field_syn_type,
            is_pk_key: false,
            is_sk_key: false,
            ignore: false,
            is_option,
            is_serde: false,
        }
    }

    fn mark_as_pk(&mut self) {
        if self.is_sk_key {
            panic!("Field is already marked as sk.");
        }
        self.is_pk_key = true;
    }

    fn mark_as_sk(&mut self) {
        if self.is_sk_key {
            panic!("Field is already marked as pk.");
        }
        self.is_sk_key = true;
    }

    fn mark_as_ignored(&mut self) {
        self.ignore = true;
    }

    fn mark_as_serde(&mut self) {
        self.is_serde = true;
    }

    fn set_key(&mut self, key: String) {
        self.key = key.replace("\"", "");
    }

    fn get_key(&self) -> Key {
        if self.is_pk_key {
            return Key::Pk(self.key.clone());
        } else if self.is_sk_key {
            return Key::Sk(self.key.clone());
        }

        return Key::Key(self.key.clone());
    }
}

impl From<&Field> for FieldInfo {
    fn from(field: &Field) -> Self {
        let mut field_scan = FieldScan::new(&field);

        for attribute in &field.attrs {
            if !attribute.path().is_ident("dynorow") {
                continue;
            }
            let _ = attribute.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    return Ok(());
                };
                match ident.to_string().as_str() {
                    "pk" => field_scan.mark_as_pk(),
                    "sk" => field_scan.mark_as_sk(),
                    "key" => {
                        let Ok(key) = meta.value() else {
                            panic!("Error while getting key value.");
                        };
                        field_scan.set_key(key.to_string())
                    }
                    "ignore" => field_scan.mark_as_ignored(),
                    "serde" => field_scan.mark_as_serde(),
                    _ => {}
                }

                return Ok(());
            });
        }

        return field_scan.into();
    }
}
