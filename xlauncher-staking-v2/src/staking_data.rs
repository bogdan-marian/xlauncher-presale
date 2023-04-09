use multiversx_sc::{
    api::ManagedTypeApi,
    types::{BigUint, TokenIdentifier, ManagedAddress, ManagedVec},
};


multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone)]
pub struct StakingSettings<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
}

// it keeps track at global level the total staked value for each pool
#[derive(TypeAbi, TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone)]
pub struct PoolData<M: ManagedTypeApi> {
    pub pool_id: usize,
    pub pool_total_xlh: BigUint<M>,
}

// it keeps track at client level the client staked value for each pool
// and the total sft staked
pub struct ClientData<M: ManagedTypeApi> {
    pub sft_amount: usize,
    pub xlh_data: ManagedVec<M, ClientXlhData<M>>,
}

#[derive(TypeAbi, TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone)]
pub struct ClientXlhData<M: ManagedTypeApi> {
    pub pull_id: usize,
    pub xlh_amount: BigUint<M>,
    pub time_stamp: u64, // always needs to be modified when amount gets modified
}

#[derive(TypeAbi, TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone)]
pub struct ClientXlhUnstakeData<M: ManagedTypeApi> {
    pub total_unstake_amount: BigUint<M>,
    pub requested_amount: BigUint<M>,
    pub requested_time_stamp: u64,
    pub free_after_time_stamp: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, ManagedVecItem, NestedEncode, NestedDecode, Clone)]
pub struct ClientSftUnstakeData<M: ManagedTypeApi> {
    pub unstake_amount: BigUint<M>,
    pub requested_time_stamp: u64,
    pub free_after_time_stamp: u64,
}
