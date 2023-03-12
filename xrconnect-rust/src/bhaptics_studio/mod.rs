use crate::{
    haptics::{
        player::{ HapticPlayer },
    },
    bhaptics_studio::tact::{
        PlayerRequest, PlayerRegisterRequest, PlayerSubmitRequest,
    },
};

pub mod tact;
pub mod server;

mod ws;

trait BHapticsStudioPlayer {
    fn handle_request(&self, request: PlayerRequest) -> Result<(), ()>;
}

impl BHapticsStudioPlayer for HapticPlayer {
    fn handle_request(&self, request: PlayerRequest) -> Result<(), ()> {
        match request {
            PlayerRequest::Register(registers) => {
                for register in registers {
                    self.handle_register_request(register)?;
                }
            }
            PlayerRequest::Submit(submits) => {
                for submit in submits {
                    self.handle_submit_request(submit)?;
                }
            }
        }

        Ok(())
    }
}

impl HapticPlayer {
    fn handle_register_request(&self, request: PlayerRegisterRequest) -> Result<(), ()> {
        Ok(())
    }
    fn handle_submit_request(&self, request: PlayerSubmitRequest) -> Result<(), ()> {
        Ok(())
    }
}