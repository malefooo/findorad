//! Voting logics
//!
//! deletate    -> increase voting power
//! un-delegate -> decrease voting power
//!

use crate::{
    delegate::{execute_delegate, DelegateOp},
    undelegate::{execute_undelegate, UnDelegateOp},
    validator_keys::ValidatorPublicKey,
};
use abcf::{
    bs3::{MapStore, ValueStore},
    tm_protos::abci::ValidatorUpdate,
    Error,
};
use libfindora::staking::{
    voting::{Amount, Power, MAX_POWER_PERCENT_PER_VALIDATOR, MAX_TOTAL_POWER},
    Operation, StakingInfo, TendermintAddress,
};
use std::collections::BTreeMap;
use zei::xfr::sig::XfrPublicKey;

pub fn execute_staking(
    info: &StakingInfo,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<XfrPublicKey, Amount>,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<XfrPublicKey, Amount>>,
    powers: &mut impl MapStore<TendermintAddress, Power>,
    validator_staker: &mut impl MapStore<TendermintAddress, XfrPublicKey>,
    validator_addr_pubkey: &mut impl MapStore<TendermintAddress, ValidatorPublicKey>,
    delegation_info: &mut impl MapStore<XfrPublicKey, BTreeMap<TendermintAddress, Amount>>,
) -> abcf::Result<Vec<ValidatorUpdate>> {
    match &info.operation {
        Operation::Delegate(d) => {
            let op = if let Some(pubkey) = &d.validator {
                DelegateOp {
                    delegator: info.delegator.clone(),
                    validator_pubkey: Some(
                        ValidatorPublicKey::from_crypto_publickey(pubkey).unwrap(),
                    ),
                    validator_address: d.address.clone(),
                    amount: info.amount,
                    memo: d.memo.clone(),
                }
            } else {
                DelegateOp {
                    delegator: info.delegator.clone(),
                    validator_pubkey: None,
                    validator_address: d.address.clone(),
                    amount: info.amount,
                    memo: d.memo.clone(),
                }
            };

            return execute_delegate(
                op,
                global_power,
                delegation_amount,
                delegators,
                powers,
                validator_staker,
                validator_addr_pubkey,
                delegation_info,
            );
        }
        Operation::Undelegate(ud) => {
            let op = UnDelegateOp {
                delegator: info.delegator.clone(),
                validator_address: ud.address.clone(),
                amount: info.amount,
            };

            return execute_undelegate(
                op,
                global_power,
                delegation_amount,
                delegators,
                powers,
                validator_addr_pubkey,
                delegation_info,
            );
        }
    }
}

/// check validatro's power rules
/// A single validator's power percentage MUST NOT > MAX_POWER_PERCENT_PER_VALIDATOR
pub fn validator_power_rules(
    current_power: Power,
    current_global_power: Power,
) -> abcf::Result<bool> {
    if (current_power as u128)
        .checked_mul(MAX_POWER_PERCENT_PER_VALIDATOR[1])
        .unwrap()
        > MAX_POWER_PERCENT_PER_VALIDATOR[0]
            .checked_mul(current_global_power as u128)
            .unwrap()
    {
        return Err(Error::ABCIApplicationError(
            90001,
            "validator power overflow".to_owned(),
        ));
    }

    Ok(true)
}

/// check global power rules
/// after delegate operation, new global power MUST NOT > MAX_TOTAL_POWER
pub fn global_power_rules(current_global_power: Power) -> abcf::Result<bool> {
    if MAX_TOTAL_POWER < current_global_power {
        return Err(Error::ABCIApplicationError(
            90001,
            "global power overflow".to_owned(),
        ));
    }

    Ok(true)
}
