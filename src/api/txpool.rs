//! `Txpool` namespace

use jsonrpc_core::Value;

use crate::{
    api::Namespace,
    helpers::CallFuture,
    types::{TxpoolContentInfo, TxpoolContentFromInfo, TxpoolInspectInfo, TxpoolStatus},
    Transport,
};

/// `Txpool` namespace
#[derive(Debug, Clone)]
pub struct Txpool<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Txpool<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Txpool { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Txpool<T> {
    /// returns txpool content info
    pub fn content(&self) -> CallFuture<TxpoolContentInfo, T::Out> {
        CallFuture::new(self.transport.execute("txpool_content", vec![]))
    }

    /// returns txpool content info
    pub fn content_from(&self, address: &str) -> CallFuture<TxpoolContentFromInfo, T::Out> {
        let argval = Value::String(address.to_string());
        CallFuture::new(self.transport.execute("txpool_contentFrom", vec![argval]))
    }


    /// returns txpool inspect info
    pub fn inspect(&self) -> CallFuture<TxpoolInspectInfo, T::Out> {
        CallFuture::new(self.transport.execute("txpool_inspect", vec![]))
    }

    /// returns txpool status
    pub fn status(&self) -> CallFuture<TxpoolStatus, T::Out> {
        CallFuture::new(self.transport.execute("txpool_status", vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::Txpool;
    use crate::{
        api::Namespace,
        types::{TxpoolContentInfo, TxpoolInspectInfo, TxpoolStatus},
    };

    const EXAMPLE_CONTENT_INFO: &str = r#"{
        "pending": {
            "0x0216d5032f356960cd3749c31ab34eeff21b3395": {
              "806": {
                "blockHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "blockNumber": null,
                "from": "0x0216d5032f356960cd3749c31ab34eeff21b3395",
                "gas": "0x5208",
                "gasPrice": "0xba43b7400",
                "hash": "0xaf953a2d01f55cfe080c0c94150a60105e8ac3d51153058a1f03dd239dd08586",
                "input": "0x",
                "nonce": "0x326",
                "to": "0x7f69a91a3cf4be60020fb58b893b7cbb65376db8",
                "transactionIndex": null,
                "value": "0x19a99f0cf456000"
              }
            },
            "0x24d407e5a0b506e1cb2fae163100b5de01f5193c": {
              "34": {
                "blockHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "blockNumber": null,
                "from": "0x24d407e5a0b506e1cb2fae163100b5de01f5193c",
                "gas": "0x44c72",
                "gasPrice": "0x4a817c800",
                "hash": "0xb5b8b853af32226755a65ba0602f7ed0e8be2211516153b75e9ed640a7d359fe",
                "input": "0xb61d27f600000000000000000000000024d407e5a0b506e1cb2fae163100b5de01f5193c00000000000000000000000000000000000000000000000053444835ec580000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                "nonce": "0x22",
                "to": "0x7320785200f74861b69c49e4ab32399a71b34f1a",
                "transactionIndex": null,
                "value": "0x0"
              }
            }
          },
          "queued": {
            "0x976a3fc5d6f7d259ebfb4cc2ae75115475e9867c": {
              "3": {
                "blockHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "blockNumber": null,
                "from": "0x976a3fc5d6f7d259ebfb4cc2ae75115475e9867c",
                "gas": "0x15f90",
                "gasPrice": "0x4a817c800",
                "hash": "0x57b30c59fc39a50e1cba90e3099286dfa5aaf60294a629240b5bbec6e2e66576",
                "input": "0x",
                "nonce": "0x3",
                "to": "0x346fb27de7e7370008f5da379f74dd49f5f2f80f",
                "transactionIndex": null,
                "value": "0x1f161421c8e0000"
              }
            },
            "0x9b11bf0459b0c4b2f87f8cebca4cfc26f294b63a": {
              "2": {
                "blockHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "blockNumber": null,
                "from": "0x9b11bf0459b0c4b2f87f8cebca4cfc26f294b63a",
                "gas": "0x15f90",
                "gasPrice": "0xba43b7400",
                "hash": "0x3a3c0698552eec2455ed3190eac3996feccc806970a4a056106deaf6ceb1e5e3",
                "input": "0x",
                "nonce": "0x2",
                "to": "0x24a461f25ee6a318bdef7f33de634a67bb67ac9d",
                "transactionIndex": null,
                "value": "0xebec21ee1da40000"
              },
              "6": {
                "blockHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
                "blockNumber": null,
                "from": "0x9b11bf0459b0c4b2f87f8cebca4cfc26f294b63a",
                "gas": "0x15f90",
                "gasPrice": "0x4a817c800",
                "hash": "0xbbcd1e45eae3b859203a04be7d6e1d7b03b222ec1d66dfcc8011dd39794b147e",
                "input": "0x",
                "nonce": "0x6",
                "to": "0x6368f3f8c2b42435d6c136757382e4a59436a681",
                "transactionIndex": null,
                "value": "0xf9a951af55470000"
              }
            }
          }
        }"#;

    const EXAMPLE_INSPECT_INFO: &str = r#"{
        "pending": {
          "0x26588a9301b0428d95e6fc3a5024fce8bec12d51": {
            "31813": "0x3375ee30428b2a71c428afa5e89e427905f95f7e: 0 wei + 500000 × 20000000000 gas"
          },
          "0x2a65aca4d5fc5b5c859090a6c34d164135398226": {
            "563662": "0x958c1fa64b34db746925c6f8a3dd81128e40355e: 1051546810000000000 wei + 90000 × 20000000000 gas",
            "563663": "0x77517b1491a0299a44d668473411676f94e97e34: 1051190740000000000 wei + 90000 × 20000000000 gas",
            "563664": "0x3e2a7fe169c8f8eee251bb00d9fb6d304ce07d3a: 1050828950000000000 wei + 90000 × 20000000000 gas",
            "563665": "0xaf6c4695da477f8c663ea2d8b768ad82cb6a8522: 1050544770000000000 wei + 90000 × 20000000000 gas",
            "563666": "0x139b148094c50f4d20b01caf21b85edb711574db: 1048598530000000000 wei + 90000 × 20000000000 gas",
            "563667": "0x48b3bd66770b0d1eecefce090dafee36257538ae: 1048367260000000000 wei + 90000 × 20000000000 gas",
            "563668": "0x468569500925d53e06dd0993014ad166fd7dd381: 1048126690000000000 wei + 90000 × 20000000000 gas",
            "563669": "0x3dcb4c90477a4b8ff7190b79b524773cbe3be661: 1047965690000000000 wei + 90000 × 20000000000 gas",
            "563670": "0x6dfef5bc94b031407ffe71ae8076ca0fbf190963: 1047859050000000000 wei + 90000 × 20000000000 gas"
          },
          "0x9174e688d7de157c5c0583df424eaab2676ac162": {
            "3": "0xbb9bc244d798123fde783fcc1c72d3bb8c189413: 30000000000000000000 wei + 85000 × 21000000000 gas"
          },
          "0xb18f9d01323e150096650ab989cfecd39d757aec": {
            "777": "0xcd79c72690750f079ae6ab6ccd7e7aedc03c7720: 0 wei + 1000000 × 20000000000 gas"
          },
          "0xb2916c870cf66967b6510b76c07e9d13a5d23514": {
            "2": "0x576f25199d60982a8f31a8dff4da8acb982e6aba: 26000000000000000000 wei + 90000 × 20000000000 gas"
          },
          "0xbc0ca4f217e052753614d6b019948824d0d8688b": {
            "0": "0x2910543af39aba0cd09dbb2d50200b3e800a63d2: 1000000000000000000 wei + 50000 × 1171602790622 gas"
          },
          "0xea674fdde714fd979de3edf0f56aa9716b898ec8": {
            "70148": "0xe39c55ead9f997f7fa20ebe40fb4649943d7db66: 1000767667434026200 wei + 90000 × 20000000000 gas"
          }
        },
        "queued": {
          "0x0f6000de1578619320aba5e392706b131fb1de6f": {
            "6": "0x8383534d0bcd0186d326c993031311c0ac0d9b2d: 9000000000000000000 wei + 21000 × 20000000000 gas"
          },
          "0x5b30608c678e1ac464a8994c3b33e5cdf3497112": {
            "6": "0x9773547e27f8303c87089dc42d9288aa2b9d8f06: 50000000000000000000 wei + 90000 × 50000000000 gas"
          },
          "0x976a3fc5d6f7d259ebfb4cc2ae75115475e9867c": {
            "3": "0x346fb27de7e7370008f5da379f74dd49f5f2f80f: 140000000000000000 wei + 90000 × 20000000000 gas"
          },
          "0x9b11bf0459b0c4b2f87f8cebca4cfc26f294b63a": {
            "2": "0x24a461f25ee6a318bdef7f33de634a67bb67ac9d: 17000000000000000000 wei + 90000 × 50000000000 gas",
            "6": "0x6368f3f8c2b42435d6c136757382e4a59436a681: 17990000000000000000 wei + 90000 × 20000000000 gas",
            "7": "0x6368f3f8c2b42435d6c136757382e4a59436a681: 17900000000000000000 wei + 90000 × 20000000000 gas"
          }
        }
      }"#;

    const EXAMPLE_STATUS: &str = r#"{
        "pending": "0xa",
        "queued": "0x7"
    }"#;

    rpc_test! (
      Txpool:content => "txpool_content";
      ::serde_json::from_str(EXAMPLE_CONTENT_INFO).unwrap()
      => ::serde_json::from_str::<TxpoolContentInfo>(EXAMPLE_CONTENT_INFO).unwrap()
    );

    rpc_test! (
      Txpool:inspect => "txpool_inspect";
      ::serde_json::from_str(EXAMPLE_INSPECT_INFO).unwrap()
      => ::serde_json::from_str::<TxpoolInspectInfo>(EXAMPLE_INSPECT_INFO).unwrap()
    );

    rpc_test! (
      Txpool:status => "txpool_status";
      ::serde_json::from_str(EXAMPLE_STATUS).unwrap()
      => ::serde_json::from_str::<TxpoolStatus>(EXAMPLE_STATUS).unwrap()
    );
}
