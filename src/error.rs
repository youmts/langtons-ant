use sdl2::{ttf::InitError, video::WindowBuildError, IntegerOrSdlError};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("sdl2 initialization error: {0}")]
    SDL2Init(String),

    #[error("sdl2 video initialization error: {0}")]
    SDL2VideoInit(String),

    #[error("sdl2 ttf initialization error: {0}")]
    SDL2TTFInit(InitError),

    #[error("load font error: {0}")]
    LoadFont(String),

    #[error("window build error: {0}")]
    WindowBuild(WindowBuildError),

    #[error("a sdl2 error: {0}")]
    IntegerOrSDL2(IntegerOrSdlError),

    #[error("a sdl2 error: {0}")]
    SDL2String(String),
}
