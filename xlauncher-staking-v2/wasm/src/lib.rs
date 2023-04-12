// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    xlauncher_staking
    (
        setContractSettings
        createNewPool
        fundWithRewords
        stakeXlh
        unstakeSft
        unstakeXlh
        claimUnstakedXlhValue
        stakeSft
        claimRewards
        getContractSettings
        getSftSettings
        getTotalStakedData
        getClientState
        getUnstakeXlhState
        getUnstakeSftState
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
