spec aptos_framework::dkg {

    spec module {
        use aptos_framework::chain_status;
        invariant [suspendable] chain_status::is_operating() ==> exists<DKGState>(@aptos_framework);
    }

    spec initialize(aptos_framework: &signer) {
        use std::signer;
        let aptos_framework_addr = signer::address_of(aptos_framework);
        aborts_if aptos_framework_addr != @aptos_framework;
    }

    spec on_async_reconfig_start() {
        aborts_if !exists<DKGState>(@aptos_framework);
        aborts_if !exists<timestamp::CurrentTimeMicroseconds>(@aptos_framework);
    }

    spec finish(transcript: vector<u8>) {
        use std::option;
        requires exists<DKGState>(@aptos_framework);
        requires option::is_some(global<DKGState>(@aptos_framework).in_progress);
        aborts_if false;
    }

    spec fun has_incomplete_session(): bool {
        if (exists<DKGState>(@aptos_framework)) {
            option::spec_is_some(global<DKGState>(@aptos_framework).in_progress)
        } else {
            false
        }
    }

    spec try_clear_incomplete_session(fx: &signer) {
        use std::signer;
        let addr = signer::address_of(fx);
        aborts_if addr != @aptos_framework;
    }

    spec incomplete_session(): Option<DKGSessionState> {
        aborts_if false;
    }
}
