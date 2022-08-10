use bytes::Bytes;
use prost::Message;

use crate::error::HederaError;
use crate::node_address::NodeAddress;
use crate::proto::services::NodeAddressBook as PbNodeAddressBook;

pub struct NodeAddressBook {
    pub node_addresses: Vec<NodeAddress>,
}

impl NodeAddressBook {
    #[allow(dead_code)]
    pub fn new() -> NodeAddressBook {
        NodeAddressBook {
            node_addresses: Vec::new(),
        }
    }

    pub fn from_proto_bytes(bytes: Vec<u8>) -> Result<NodeAddressBook, HederaError> {
        match PbNodeAddressBook::decode(Bytes::from(bytes)) {
            Ok(proto) => Ok(proto.into()),
            Err(_) => Err(HederaError::NodeAddressBookDeserialize),
        }
    }
}

impl From<PbNodeAddressBook> for NodeAddressBook {
    fn from(pb_node_address_book: PbNodeAddressBook) -> NodeAddressBook {
        NodeAddressBook {
            node_addresses: pb_node_address_book
                .node_address
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<NodeAddress>>(),
        }
    }
}
