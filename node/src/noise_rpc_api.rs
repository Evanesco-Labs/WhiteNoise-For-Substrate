use jsonrpc_core::{
    Error as RpcError,
    futures::future::{self as rpc_future},
};
use jsonrpc_derive::rpc;
use futures::{FutureExt as _, TryFutureExt as _};
use whitenoisers::network::node::Node;

type FutureResult<T> = Box<dyn rpc_future::Future<Item=T, Error=RpcError> + Send>;

#[rpc]
pub trait NoiseRpcApi {
    #[rpc(name = "get_main_nets")]
    fn get_main_nets(&self, num: i32) -> FutureResult<Vec<String>>;
}

pub struct NoiseRpc {
    pub node: Node
}

impl NoiseRpcApi for NoiseRpc {
    fn get_main_nets(&self, num: i32) -> FutureResult<Vec<String>> {
        let node_request_sender = self.node.node_request_sender.clone();
        let fnn = async move {
            let (sender, receiver) = futures::channel::oneshot::channel();
            let get_main_nets = whitenoisers::network::whitenoise_behaviour::GetMainNets {
                command_id: String::from(""),
                remote_peer_id: libp2p::PeerId::random(),
                num: num,
                sender: sender,
            };
            node_request_sender.unbounded_send(whitenoisers::network::whitenoise_behaviour::NodeRequest::GetMainNetsRequest(get_main_nets));
            let nodeinfos_res = receiver.await;
            let zzz = nodeinfos_res.unwrap();
            let id_: Vec<String> = zzz.iter().map(|x| x.id.clone()).collect();
            return Ok(id_);
        }.boxed();
        Box::new(fnn.compat())
    }
}
