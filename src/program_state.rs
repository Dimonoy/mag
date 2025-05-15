use crate::{canvas::AppCanvas, texture::TextureWrapper};

pub struct ProgramRuntime {
    state: Option<Box<dyn ProgramState>>,
    pub canvas: AppCanvas,
}

#[derive(Debug)]
pub enum ProgramExitType {
    Close,
    Exit,
}

impl ProgramRuntime {
    pub fn new(canvas: AppCanvas) -> Self {
        Self { canvas, state: None }
    }

    pub fn set_state_continue(&mut self) {
        self.state = Some(Box::new(ProgramContinue));
    }

    pub fn set_state_force_update(&mut self) {
        self.state = Some(Box::new(ProgramForceUpdate));
    }

    pub fn set_state_close(&mut self) {
        self.state = Some(Box::new(ProgramClose));
    }

    pub fn set_state_exit(&mut self) {
        self.state = Some(Box::new(ProgramExit));
    }

    pub fn execute(&mut self, texture_wrapper: &TextureWrapper<'_>) -> Result<(), ProgramExitType> {
        if let Some(state) = self.state.take() {
            state.execute(&mut self.canvas, texture_wrapper)?;
        }
        self.set_state_continue();

        Ok(())
    }
}

trait ProgramState {
    fn execute(self: Box<Self>, canvas: &mut AppCanvas, texture_wrapper: &TextureWrapper) -> Result<(), ProgramExitType>;
    fn to_string(&self) -> String;
}

impl std::fmt::Debug for dyn ProgramState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
struct ProgramContinue;

impl ProgramState for ProgramContinue {
    fn execute(self: Box<Self>, _canvas: &mut AppCanvas, _texture_wrapper: &TextureWrapper) -> Result<(), ProgramExitType> {
        Ok(())
    }

    fn to_string(&self) -> String {
        String::from("ProgramContinue")
    }
}

#[derive(Debug)]
struct ProgramForceUpdate;

impl ProgramState for ProgramForceUpdate {
    fn execute(self: Box<Self>, canvas: &mut AppCanvas, texture_wrapper: &TextureWrapper) -> Result<(), ProgramExitType> {
        if let Err(err_msg) = canvas.update(&texture_wrapper) {
            #[cfg(feature = "dev")]
            log::error!(err_msg);
            eprintln!("{}", err_msg);
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        String::from("ProgramForceUpdate")
    }
}

#[derive(Debug)]
struct ProgramClose;

impl ProgramState for ProgramClose {
    fn execute(self: Box<Self>, _canvas: &mut AppCanvas, _texture_wrapper: &TextureWrapper) -> Result<(), ProgramExitType> {
        Err(ProgramExitType::Close)
    }

    fn to_string(&self) -> String {
        String::from("ProgramClose")
    }
}

#[derive(Debug)]
struct ProgramExit;

impl ProgramState for ProgramExit {
    fn execute(self: Box<Self>, _canvas: &mut AppCanvas, _texture_wrapper: &TextureWrapper) -> Result<(), ProgramExitType> {
        Err(ProgramExitType::Exit)
    }

    fn to_string(&self) -> String {
        String::from("ProgramExit")
    }
}
