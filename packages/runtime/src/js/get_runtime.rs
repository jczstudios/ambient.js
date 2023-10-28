use ambient_api::anyhow::Result;

use javy::Runtime;
use crate::js::inject_console;

use super::inject_ambient;


/// Returns the runtime needed to execute JS, adding the needed global objects
/// to interact with the Ambient API.
pub fn get_runtime() -> Result<Runtime> {
    
    let runtime = Runtime::default();

    let context = runtime.context();

    inject_console(context)?;

    inject_ambient(context)?;

    Ok(runtime)
}