use crate::Type;

use std::any::TypeId;
use std::cmp::PartialEq;

// TODO: Don't make all these fields public

pub struct Fields {
    pub type_id: TypeId,
    pub definitions: &'static [FieldDefinition],
}

pub struct Field {
    pub type_id: TypeId,
    pub definition: &'static FieldDefinition,
}

pub struct FieldDefinition {
    pub name: &'static str,
    pub ty: Type,
}

impl Fields {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Field> + 'a {
        self.definitions.into_iter().map(move |definition| Field {
            type_id: self.type_id,
            definition,
        })
    }
}

impl Field {
    pub fn name(&self) -> &str {
        self.definition.name
    }
}

impl PartialEq for &'static FieldDefinition {
    fn eq(&self, other: &&'static FieldDefinition) -> bool {
        std::ptr::eq(*self, *other)
    }
}

/*
THIS IS IGNORED


use std::any::TypeId;

pub struct Fields {
    type_id: TypeId,
    definitions: &'static [FieldDefinition]
}

pub struct Field {
    type_id: TypeId,
    definition: &'static FieldDefinition,
}

/// Field definition
struct FieldDefinition {
    name: &'static str,
    ty: Type,
}

enum Type {
    u8,
    u16,
    u32,
    u64,
    String,
    // ... more here
    Valuable,
    Mappable,
    Listable,
}

struct Value<'a> {

}

enum Kind<'a> {
    U8(u8),
    // ... more here
    Valuable(&'a dyn Valuable),
    Mappable(&'a dyn Mappable),
    Listable(&'a dyn Listable),
}

pub trait Valuable {
    fn fields(&self) -> Fields;

    fn field_by_name(&self, name: &str) -> Option<Field>;

    fn get(&self, field: &Field) -> Option<Value<'_>>;
}

pub trait Listable {
    fn get(&self, index: usize) -> Option<Value<'_>>;
}

pub trait Mappable {
    fn get(&self, key: &Value<'_>) -> Option<Value<'_>>;
}

struct HelloWorld {
    hello: &'static str,
    world: u16,
}

static HELLO_WORLD_FIELDS: &'static [FieldDefinition] => &[
    FieldDefinition { name: "hello", ty: Type::String },
    FieldDefinition { name: "world", ty: Type::U16 },
];

impl Valuable for HelloWorld {
    fn fields(&self) -> Fields {
        Fields {
            ty: TypeId::of::<Self>::(),
            definitions: HELLO_WORLD_FIELDS,
        }
    }

    fn field_by_name(&self, name: &str) -> Option<Field> {
        unimplemented!();
    }

    fn get(&self, field: &Field) -> Option<Value<'_>> {
        // This is a bit fuzzy
        match field {
            HELLO_WORLD_FIELDS[0] => Some(Value::string(&self.hello)),
            HELLO_WORLD_FIELDS[1] => Some(Value::from_u16(&self.world)),
            _ => None,
        }
    }
}
*/