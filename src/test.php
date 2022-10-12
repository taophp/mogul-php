<?php
/*
/// let original = MogulHandler::new(Uuid::new_v4(), Uuid::new_v4(), vec![Uuid::new_v4(),Uuid::new_v4()]);
/// let challenger_not_same = MogulHandler::new (Uuid::new_v4(), Uuid::new_v4(), vec![Uuid::new_v4(),Uuid::new_v4()]);
/// let challenger_older = MogulHandler::new(original.get_uuid(), original.get_history()[0], vec![]);
/// let challenger_diverge_older = MogulHandler::new(original.get_uuid(), Uuid::new_v4(), vec![original.get_history()[0]]);
/// let challenger_diverge =  MogulHandler::new(original.get_uuid(), Uuid::new_v4(), vec![original.get_history()[0], Uuid::new_v4(), Uuid::new_v4()]);
/// let mut challenger_allowed =  MogulHandler::new(original.get_uuid(), Uuid::new_v4(), original.get_history());
/// challenger_allowed.define_new_state();
/// 
/// assert_eq!(mogul_allow_update(&original, &challenger_not_same), Err(MogulError::NotVersionsOfTheSameObject));
/// assert_eq!(mogul_allow_update(&original, &challenger_older), Err(MogulError::ChallengerOlderThanOriginal));
/// assert_eq!(mogul_allow_update(&original, &challenger_diverge_older), Err(MogulError::Conflict));
/// assert_eq!(mogul_allow_update(&original, &challenger_diverge), Err(MogulError::Conflict));
/// assert_eq!(mogul_allow_update(&original, &original), Err(MogulError::IdenticalVersions));
/// assert_eq!(mogul_allow_update(&original, &challenger_allowed), Ok(()));
/// assert_eq!(original.get_uuid(), original.get_uuid());
/*/

$original = mogule_handler_new_start();
var_dump($original->serialize());
$original->defineNewState();
var_dump($original->serialize());
$challengerNotSame = mogule_handler_new_start();
var_dump(MogulHl::allow_merge($original,$challengerNotSame));

