//! # Alkahest-session
#![cfg_attr(not(test), no_std)]

/// The state of a session.
pub enum SessionState {
    ///
    Validating,
    ///
    Compiling,
    ///
    Linking,
    ///
    Ready,
    ///
    Failed,
}

pub struct Session {
    pub module_id: u32,
    state: SessionState,
}

impl Session {
    pub fn new(module_id: u32) -> Self {
        Self {
            module_id,
            state: SessionState::Validating,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_session() {
        let _ = Session::new(123);       
    }
}