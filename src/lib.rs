#![warn(clippy::pedantic)]

pub mod app;

mod camera;

mod chip8;

mod display;

mod emulator;

#[cfg(feature = "editor")]
mod editor;

mod window;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
