use super::*;
use alloc::borrow::{Cow, ToOwned};
use alloc::string::String;
use schemars::_private::apply_metadata;
use schemars::_serde_json::Value;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::{InstanceType, Metadata, Schema};

impl JsonSchema for Version {
    fn schema_name() -> String {
        "Version".to_owned()
    }
    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed(concat!(module_path!(), "::", "Version"))
    }
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema = gen.subschema_for::<String>();
        match &mut schema {
            Schema::Object(schema_object) => {
                let validation = schema_object.string();
                validation.pattern = Some(r"[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+".to_string());
            }
            _ => {}
        }
        let schema = apply_metadata(schema, Metadata { default: Some(Value::String("0.0.0.0".to_string())), ..Default::default() });
        schema
    }
}
