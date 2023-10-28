use javy::quickjs::{
        JSValue,
        JSContextRef
    };

use anyhow::Result;

/// Injects the console into the global JS object.
pub fn inject_console(context: &JSContextRef) -> Result<()> {
    let global = context.global_object()?;

    let console_object = context.object_value()?;

    console_object.set_property("log", context.wrap_callback(move | _ctx, _this, args| {
        if !args[0].is_str() {
            
        }

        let log_message = args[0].as_str()?;

        println!("{}", log_message);

        Ok(JSValue::Undefined)
    })?
    )?;

    global.set_property("console", console_object)?;

    Ok(())
}