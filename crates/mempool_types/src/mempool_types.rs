use mempool_infra::network_component::NetworkComponent;
use starknet_api::core::{ContractAddress, Nonce};
use starknet_api::internal_transaction::InternalTransaction;

#[derive(Clone, Copy, Debug, Default)]
pub struct AccountState {
    pub nonce: Nonce,
    // TODO: add balance field when needed.
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Account {
    pub address: ContractAddress,
    pub state: AccountState,
}

#[derive(Debug)]
pub struct MempoolInput {
    pub tx: InternalTransaction,
    pub account: Account,
}

#[derive(Debug)]
pub enum Gateway2MempoolMessage {
    AddTx(InternalTransaction, Account),
}

pub type Mempool2GatewayMessage = ();

pub type GatewayNetworkComponent = NetworkComponent<Gateway2MempoolMessage, Mempool2GatewayMessage>;
pub type MempoolNetworkComponent = NetworkComponent<Mempool2GatewayMessage, Gateway2MempoolMessage>;