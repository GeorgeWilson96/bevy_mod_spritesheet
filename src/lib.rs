use std::error::Error;
use std::fmt;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::asset::io::{Reader, VecReader};
use bevy::prelude::*;
use bevy::utils::thiserror::Error;
use serde::Deserialize;
use crate::format::json::array::JsonArray;

mod format;
mod systems;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<SpriteSheet>()
            .init_asset_loader::<Loader>();
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct Frame<'a>(&'a str);

#[derive(Asset, TypePath, Deserialize)]
pub struct SpriteSheet(JsonArray);

/// Purposefully kept empty, so it does not cause conflict with other loaders
/// which use the `.json` extension.
pub const SUPPORTED_EXTENSIONS: &[&str] = &["json"];

#[derive(Default)]
pub struct Loader;

impl AssetLoader for Loader {
    type Asset = SpriteSheet;
    type Settings = ();
    type Error = LoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {

            let mut raw = Vec::new();

            let _ = reader
                .read_to_end(&mut raw)
                .await.map_err(|_| {LoaderError::Io})?;

            let format = serde_json::from_slice::<JsonArray>(raw.as_slice()).map_err(|_| { LoaderError::JsonParseError })?;

            info!("{:?}", format);

            Ok(SpriteSheet(format))
        })
    }

    fn extensions(&self) -> &[&str] {
        SUPPORTED_EXTENSIONS
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum LoaderError {
    Io,
    JsonParseError,
}
impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An {} error occurred", self)
    }
}
impl Error for LoaderError {}