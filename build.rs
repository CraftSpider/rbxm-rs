use std::fs::File;
use std::io;
use std::io::Write;
use serde::Deserialize;
use convert_case::{Case, Casing};

#[derive(Deserialize)]
#[derive(Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
enum Tag {
    CanYield,
    CustomLuaState,
    Deprecated,
    Hidden,
    NotBrowsable,
    NotCreatable,
    NotReplicated,
    NotScriptable,
    NoYield,
    PlayerReplicated,
    ReadOnly,
    Service,
    Settings,
    UserSettings,
    Yields,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase")]
enum SecType {
    None,
    LocalUserSecurity,
    NotAccessibleSecurity,
    PluginSecurity,
    RobloxScriptSecurity,
    RobloxSecurity,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase")]
enum ValueCategory {
    Class,
    DataType,
    Enum,
    Group,
    Primitive,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase")]
enum ThreadSafety {
    ReadSafe,
    Safe,
    Unsafe,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", untagged)]
enum Security {
    Single(SecType),
    #[serde(rename_all = "PascalCase")]
    Separate {
        read: SecType,
        write: SecType,
    },
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Serialization {
    can_load: bool,
    can_save: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct ValueType {
    category: ValueCategory,
    name: String,
}

impl ValueType {
    fn as_type(&self) -> String {
        match self.category {
            ValueCategory::Class => String::from("crate::model::data::InstanceRef"),
            ValueCategory::DataType => match &*self.name {
                "BinaryString" => String::from("Vec<u8>"),
                "ProtectedString" => String::from("String"),
                // TODO
                "QDir" | "QFont" => String::from("String"),
                "DateTime" => String::from("i64"),
                name => format!("crate::model::data::{}", name),
            }
            ValueCategory::Enum => format!("super::enums::{}", self.name),
            ValueCategory::Group => self.name.clone(),
            ValueCategory::Primitive => match &*self.name {
                "string" => String::from("String"),
                "int" => String::from("i32"),
                "int64" => String::from("i64"),
                "float" => String::from("f32"),
                "double" => String::from("f64"),
                name => name.to_string(),
            },
        }
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", tag = "MemberType")]
enum Member {
    Callback(CallbackData),
    Event(EventData),
    Function(FunctionData),
    Property(PropertyData),
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct CallbackData {
    name: String,
    parameters: Vec<Parameter>,
    return_type: ValueType,
    security: Security,
    thread_safety: ThreadSafety,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct PropertyData {
    category: String,
    name: String,
    security: Security,
    serialization: Serialization,
    thread_safety: ThreadSafety,
    value_type: ValueType,
    tags: Option<Vec<Tag>>,
}

impl PropertyData {
    fn tags(&self) -> &[Tag] {
        self.tags.as_deref().unwrap_or(&[])
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Parameter {
    default: Option<String>,
    name: String,
    #[serde(rename = "Type")]
    ty: ValueType,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct FunctionData {
    name: String,
    parameters: Vec<Parameter>,
    return_type: ValueType,
    security: Security,
    thread_safety: ThreadSafety,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct EventData {
    name: String,
    parameters: Vec<Parameter>,
    security: Security,
    thread_safety: ThreadSafety,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct RbxClass {
    members: Vec<Member>,
    memory_category: String,
    name: String,
    superclass: String,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct Variant {
    name: String,
    value: u32,
    legacy_names: Option<Vec<String>>,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct RbxEnum {
    name: String,
    items: Vec<Variant>,
    tags: Option<Vec<Tag>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
struct RobloxApi {
    classes: Vec<RbxClass>,
    enums: Vec<RbxEnum>,
    version: u64,
}

fn get_api() -> Result<RobloxApi, reqwest::Error> {
    let cur_version = reqwest::blocking::get("http://setup.roblox.com/versionQTStudio")?
        .text()?;

    let api_json = reqwest::blocking::get(format!("http://setup.roblox.com/{cur_version}-API-Dump.json"))?
        .json::<RobloxApi>()?;

    Ok(api_json)
}

fn write_instance(file: &mut File, variants: Vec<String>) -> Result<(), io::Error> {
    let variants = variants.into_iter()
        .filter_map(|name| {
            if name == "Instance" {
                None
            } else {
                Some(format!("{0}({0}),", name))
            }
        })
        .collect::<Vec<_>>()
        .join("\n    ");

    write!(file, "\
use alloc::collections::BTreeMap;
use alloc::boxed::Box;
use rbxm_proc::InstanceExtra;
use super::classes::*;
use crate::model::Property;

#[non_exhaustive]
#[derive(Debug, Clone, InstanceExtra)]
pub enum Instance {{
    {variants}

    /// Any unhandled class type falls here, with its name and properties preserved but
    /// uninterpreted
    // This must remain the last item in the enum for code generation
    Other(String, BTreeMap<String, Property>),
}}
")
}

fn write_class_header(file: &mut File) -> Result<(), io::Error> {
    write!(file, "\
use rbxm_proc::{{Inherits, PropertyConvert}};
")
}

fn write_class(file: &mut File, class: &RbxClass) -> Result<(), io::Error> {
    let members = class.members
        .iter()
        .filter_map(|member| {
            if let Member::Property(prop) = member {
                if !prop.name.chars().all(char::is_alphanumeric) {
                    return None;
                }
                if prop.tags().contains(&Tag::NotReplicated) {
                    return None;
                }

                let ty = prop.value_type.as_type();

                Some(match &*prop.name {
                    "Loop" => format!("#[propname = \"Loop\"] pub loop_seq: {ty},"),
                    "CFrame" => format!("#[propname = \"CFrame\"] pub cframe: {ty},"),
                    "Type" => format!("#[propname = \"Type\"] pub ty: {ty},"),
                    "userId" => format!("#[propname = \"userId\"] pub _user_id: {ty},"),
                    name => format!("pub {}: {ty},", name.to_case(Case::Snake)),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n    ");

    if class.name == "Instance" {
        return Ok(());
    }

    let name = &class.name;

    let parent = if class.superclass == "Instance" {
        format!("pub base: crate::model::classes::Base,")
    } else {
        format!("pub {}: {},", class.superclass.to_case(Case::Snake), class.superclass)
    };

    write!(file, "\
#[doc = doc_link!(\"class/{name}\")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct {name} {{
    {parent}
    {members}
}}
")
}

fn write_classes(classes: &[RbxClass]) -> Result<(), io::Error> {
    let mut instance_variants = Vec::new();

    std::fs::create_dir_all("src/__gen")?;
    let mut instance = File::create("src/__gen/instance.rs")?;
    let mut class_file = File::create("src/__gen/classes.rs")?;

    write_class_header(&mut class_file)?;

    for class in classes {
        instance_variants.push(class.name.clone());
        write_class(&mut class_file, class)?;
    }

    write_instance(&mut instance, instance_variants)?;

    Ok(())
}

fn write_enum_header(file: &mut File) -> Result<(), io::Error> {
    write!(file, "\
use rbxm_proc::EnumConvert;
")
}

fn write_enum(file: &mut File, enum_: &RbxEnum) -> Result<(), io::Error> {
    let name = &enum_.name;
    let members = enum_
        .items
        .iter()
        .map(|var| match &*var.name {
            "Self" => format!("_Self = {},", var.value),
            _ => format!("{} = {},", var.name, var.value),
        })
        .collect::<Vec<_>>()
        .join("\n    ");

    write!(file, "\
#[doc = doc_link!(\"enum/{name}\")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum {name} {{
    {members}
}}
")
}

fn write_enums(enums: &[RbxEnum]) -> Result<(), io::Error> {

    let mut enum_file = File::create("src/__gen/enums.rs")?;

    write_enum_header(&mut enum_file)?;

    for enum_ in enums {
        write_enum(&mut enum_file, enum_)?;
    }

    Ok(())
}

fn main() {
    let api = get_api().unwrap();
    write_classes(&api.classes).unwrap();
    write_enums(&api.enums).unwrap();
}
