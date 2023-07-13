use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use jsonrpsee::core::Error;
use jsonrpsee::{Methods, RpcModule};
use kakarot_rpc_core::client::api::KakarotEthApi;
use starknet::providers::Provider;

use crate::api::alchemy_api::AlchemyApiServer;
use crate::api::eth_api::EthApiServer;
use crate::servers::alchemy_rpc::AlchemyRpc;
use crate::servers::eth_rpc::KakarotEthRpc;

/// Represents RPC modules that are supported by reth
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KakarotRpcModule {
    Eth,
    Alchemy,
}

pub struct KakarotRpcModuleBuilder<P: Provider + Send + Sync + 'static> {
    modules: HashMap<KakarotRpcModule, Methods>,
    _phantom: PhantomData<P>,
}

impl<P: Provider + Send + Sync + 'static> KakarotRpcModuleBuilder<P> {
    pub fn new(kakarot_client: Arc<dyn KakarotEthApi<P>>) -> Self {
        let eth_rpc_module = KakarotEthRpc::new(kakarot_client.clone()).into_rpc();
        let alchemy_rpc_module = AlchemyRpc::new(kakarot_client).into_rpc();

        let mut modules: HashMap<KakarotRpcModule, Methods> = HashMap::new();

        modules.insert(KakarotRpcModule::Eth, eth_rpc_module.into());
        modules.insert(KakarotRpcModule::Alchemy, alchemy_rpc_module.into());

        Self { modules, _phantom: PhantomData }
    }

    pub fn rpc_module(&self) -> Result<RpcModule<()>, Error> {
        let mut rpc_module = RpcModule::new(());

        for methods in self.modules.values().cloned() {
            rpc_module.merge(methods)?;
        }

        Ok(rpc_module)
    }
}