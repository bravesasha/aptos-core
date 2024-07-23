// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_types::transaction::TransactionStatus;
use aptos_vm_logging::log_schema::AdapterLogSchema;
use aptos_vm_types::output::VMOutput;
use move_core_types::account_address::AccountAddress;
use move_core_types::value::{MoveValue, serialize_values};
use move_core_types::vm_status::{AbortLocation, StatusCode, VMStatus};
use move_vm_runtime::module_traversal::{TraversalContext, TraversalStorage};
use move_vm_types::gas::UnmeteredGasMeter;
use crate::aptos_vm::{get_or_vm_startup_failure, get_system_transaction_output};
use crate::AptosVM;
use crate::errors::expect_only_successful_execution;
use crate::move_vm_ext::{AptosMoveResolver, SessionId};
use crate::system_module_names::{MPC_MODULE, UPDATE_STATE};
use crate::validator_txns::mpc::ExecutionFailure::{Expected, Unexpected};


#[derive(Debug)]
enum ExpectedFailure {
    // Move equivalent: `errors::invalid_argument(*)`
    EpochNotCurrent = 0x10001,
    TranscriptDeserializationFailed = 0x10002,
    TranscriptVerificationFailed = 0x10003,

    // Move equivalent: `errors::invalid_state(*)`
    MissingResourceDKGState = 0x30001,
    MissingResourceInprogressDKGSession = 0x30002,
    MissingResourceConfiguration = 0x30003,
    MissingResourceFeatures = 0x30004,
}

enum ExecutionFailure {
    Expected(ExpectedFailure),
    Unexpected(VMStatus),
}

impl AptosVM {
    pub(crate) fn process_mpc_state_update(
        &self,
        resolver: &impl AptosMoveResolver,
        log_context: &AdapterLogSchema,
        session_id: SessionId,
    ) -> Result<(VMStatus, VMOutput), VMStatus> {
        match self.process_mpc_state_update_inner(resolver, log_context, session_id) {
            Ok((vm_status, vm_output)) => Ok((vm_status, vm_output)),
            Err(Expected(failure)) => {
                Ok((
                    VMStatus::MoveAbort(AbortLocation::Script, failure as u64),
                    VMOutput::empty_with_status(TransactionStatus::Discard(StatusCode::ABORTED)),
                ))
            },
            Err(Unexpected(vm_status)) => Err(vm_status),
        }
    }

    fn process_mpc_state_update_inner(
        &self,
        resolver: &impl AptosMoveResolver,
        log_context: &AdapterLogSchema,
        session_id: SessionId,
    ) -> Result<(VMStatus, VMOutput), ExecutionFailure> {
        //mpc todo: check results
        let mut gas_meter = UnmeteredGasMeter;
        let mut session = self.new_session(resolver, session_id, None);
        let module_storage = TraversalStorage::new();

        let args = vec![];
        let vm_result = session
            .execute_function_bypass_visibility(
                &MPC_MODULE,
                UPDATE_STATE,
                vec![],
                serialize_values(&args),
                &mut gas_meter,
                &mut TraversalContext::new(&module_storage),
            )
            .map_err(|e| {
                expect_only_successful_execution(e, UPDATE_STATE.as_str(), log_context)
            });

        vm_result.map_err(|r| Unexpected(r.unwrap_err()))?;

        let output = get_system_transaction_output(
            session,
            &get_or_vm_startup_failure(&self.storage_gas_params, log_context)
                .map_err(Unexpected)?
                .change_set_configs,
        )
            .map_err(Unexpected)?;

        Ok((VMStatus::Executed, output))
    }
}
