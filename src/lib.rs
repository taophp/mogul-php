#![cfg_attr(windows, feature(abi_vectorcall))]
#[allow(unused_imports)]
use ext_php_rs::{prelude::*, convert::FromZval};
use mogul::*;
//use serde::{Serialize, Deserialize};
//#use uuid::Uuid;
//use std::{convert::From, string};


#[php_class]
pub struct MogulHl(MogulHandler);

#[php_function]
pub fn mogule_handler_new_start() -> MogulHl {
    MogulHl(MogulHandler::new_start())
}

#[php_impl]
impl MogulHl {
    pub fn define_new_state(&mut self)  {
        self.0.define_new_state();
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.0).unwrap().to_string()
    }

    pub fn allow_merge(original: &MogulHl, challenger: &MogulHl) -> PhpResult {
        match mogul_allow_update(&original.0, &challenger.0) {
            Ok(()) => Ok(()),
            Err(MogulError::ChallengerOlderThanOriginal) => {
                Err(PhpException::default("Challenger older than the original".into()))
            } ,
            Err(MogulError::Conflict) => {
                Err(PhpException::default("Conflict between the two versions".into()))
            },
            Err(MogulError::IdenticalVersions) => {
                PhpResult::Err(PhpException::default("Identical versions".into()))
            },
            Err(MogulError::NotVersionsOfTheSameObject) => {
                PhpResult::Err(PhpException::default("Versions of different objects".into()))
            },
        }
    }

    pub fn merge(original:  &MogulHl, challenger: &MogulHl) -> MogulHl {
        MogulHl(mogul_merge(&original.0, &challenger.0))
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
