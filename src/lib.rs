#![cfg_attr(windows, feature(abi_vectorcall))]
use ext_php_rs::{prelude::*, convert::FromZval};
use mogul::*;
//use serde::{Serialize, Deserialize};
//#use uuid::Uuid;
//use std::{convert::From, string};


#[php_class]
pub struct MogulHl {
    mogul: MogulHandler,
}

impl From<MogulHandler> for MogulHl {
    fn from(item: MogulHandler) -> Self {
        MogulHl {
            mogul: item,
        }
    }
}

#[php_function]
pub fn mogule_handler_new_start() -> MogulHl {
    MogulHl::from(MogulHandler::new_start())
}

#[php_impl]
impl MogulHl {
    pub fn __construct(json: String) -> Self {
        Self {
            mogul: serde_json::from_str(&json).unwrap(),
        }
    }
    pub fn define_new_state(&mut self)  {
        self.mogul.define_new_state();
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self.mogul).unwrap().to_string()
    }

    pub fn allow_merge(original: &MogulHl, challenger: &MogulHl) -> PhpResult {
        match mogul_allow_update(&original.mogul, &challenger.mogul) {
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

    pub fn merge(original:  MogulHl, challenger: MogulHl) -> MogulHl {
        MogulHl {
            mogul: mogul_merge(&original.mogul, &challenger.mogul)
        }
    }

}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
