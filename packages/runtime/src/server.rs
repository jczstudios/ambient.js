
use anyhow::Result;

mod js;
use crate::js::get_runtime;


// use std::{result, string};


use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::quad,
        transform::components::{lookat_target, translation},
    },
    prelude::*,
    anyhow
};


// use javy::{quickjs::JSValue, Runtime};

    
#[main]
pub fn main() -> Result<()> {

    let runtime = get_runtime()?;

    let context = runtime.context();

    let content = include_str!("../test.js");

    let _result = context.eval_global("hello.js", content)?;


    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(quad(), ())
        .spawn();

    println!("Hello, Ambient, from Runtime!");

    Ok(())
}
