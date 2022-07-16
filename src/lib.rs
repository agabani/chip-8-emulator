#![warn(clippy::pedantic)]

pub mod app;

mod camera;

mod display;

mod emulator;

mod editor;

mod rom;

mod window;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
