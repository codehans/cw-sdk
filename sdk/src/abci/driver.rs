use std::sync::mpsc::Receiver;

use super::{channel_send, AppCommand, ABCIError};
use crate::State;

/// The driver is a wrapper around the actual state machine. It maintains a channel with the ABCI
/// server, and performs actions or queries on the state machine on request for the ABCI server.
pub struct AppDriver {
    pub state: State,
    pub cmd_rx: Receiver<AppCommand>,
}

impl AppDriver {
    pub fn run(mut self) -> Result<(), ABCIError> {
        loop {
            match self.cmd_rx.recv()? {
                AppCommand::StoreCode {
                    wasm_byte_code,
                    result_tx,
                } => {
                    let code_id = self.state.store_code(wasm_byte_code)?;
                    channel_send(&result_tx, code_id)?;
                },
                AppCommand::QueryCode {
                    code_id,
                    result_tx,
                } => {
                    let wasm_byte_code = self.state.query_code(code_id)?;
                    channel_send(&result_tx, wasm_byte_code)?;
                }
            }
        }
    }
}