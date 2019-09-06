use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::timing::Time,
	core::transform::Transform,
	ecs::prelude::{Component, DenseVecStorage, Entity},
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
	ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const ARENA_HEIGHT: f32 = 200.0;
pub const ARENA_WIDTH: f32 = 200.0;

#[derive(Default)]
pub struct MarioBros;

impl SimpleState for MarioBros {
	
}
