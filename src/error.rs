use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("sdl2 error")]
    SDL2Error(String),
}
