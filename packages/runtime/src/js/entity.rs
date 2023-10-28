use ambient_api::prelude::Entity;
use javy::quickjs::{
    JSValue,
    JSContextRef, JSValueRef
};

use anyhow::Result;

/// https://docs.rs/ambient_api/latest/ambient_api/ecs/struct.Entity.html

/// Instantiates a new Ambient Entity and returns a pointer.
// fn construct_entity(_ctx: &JSContextRef, _this: JSValueRef, _args: &[JSValueRef]) -> Result<JSValue> {


//     Ok(JSValue::Null)
// }


// Adds the entity methods to the global JS API.
pub fn inject_entity(ambient_obj: &JSValueRef, context: &JSContextRef) -> Result<()> {

    let construct_entity = move |_ctx: &JSContextRef, _this: JSValueRef, _args: &[JSValueRef]| -> Result<JSValue> {
        let mut store = entity_store.lock().unwrap();
        // Now you can use `store` to manipulate your EntityStore

        Ok(JSValue::Null)
    };

    ambient_obj.set_property("construct_entity", context.wrap_callback(construct_entity)?)?;

    Ok(())
}