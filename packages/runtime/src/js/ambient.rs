use javy::quickjs::{
    JSValue,
    JSContextRef, JSValueRef
};

use anyhow::{Result, Ok};

use super::inject_entity;

/// Injects the Ambient API into the global JS object.
pub fn inject_ambient(context: &JSContextRef) -> Result<()> {

    let global = context.global_object()?;

    let ambient_object = context.object_value()?;

    inject_entity(&ambient_object, &context)?;

    global.set_property("ambient", ambient_object)?;

    Ok(())
}