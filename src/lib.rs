pub mod gossip {

    pub mod error {

        pub enum Error {
            AllSeedsUnavailable(&[node::Address]),
        }
    }

    pub mod message {

        pub struct Entity {
            pub address: node::Address,
        }

        pub struct WhatIsTheTea {
            pub since_version: usize,
            pub requestor: Entity,
        }

        pub enum Message {
            WhatIsTheTea(WhatIsTheTea),
        }

        pub struct NodeInfo {
            pub address: node::Address,
            pub adjacent_to: Vec<node::Address>,
        }

        pub struct Tea {
            pub source: Entity,
            pub version: usize,
            pub nodes: Vec<NodeInfo>,
        }

        pub enum Reply {
            Tea(Tea),
        }
    }

    pub mod node {

        pub struct Address {
            pub ip_addr: std::net::IpAddr,
            pub port: u16,
        }

        pub struct Data {
            pub version: usize,
        }

        pub struct Node {
            pub address: Address,
            pub since: std::time::Instant,
            pub data: Data,
        }
    }

    pub struct Handle {}

    impl Handle {
        pub fn init(
            local_address: node::Address,
            seed_list: &[node::Address],
        ) -> Result<Self, error::Error> {
            for address in seed_list {
                if address == local_address {
                    continue; // skip any seed address which is this node
                }

                // Attempt to connect to this seed
                let seed_node = match node::Node::connect(address) {
                    Ok(node) => node,
                    Err(e) => {
                        log::warn!("Failed to connect to a seed address ({address}): {e}");
                        continue;
                    }
                };

                // Attempt to fetch cluster details from this seed
                let reply = match seed_node.send(message::Message::WhatIsTheTea(message::WhatIsTheTea {
                    since_version: 0, // zero means send back everything
                    requestor: message::Entity {
                        address: local_address,
                    },
                })) {
                    Ok(reply) => reply,
                    Err(e) => {
                        log::warn!("Failed to send request to seed address ({address}): {e}");
                        continue;
                    }
                };

                // Handle a response
                match reply {
                    message::Reply::Tea(tea) => {
                    }
                    reply => {
                        log::warn!("Expected a successful response from seed address ({address}): {reply}");
                        continue;
                    }
                }
            }

            Err(error::AllSeedsUnavailable(seed_list))
        }
    }
}
