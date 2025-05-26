pub mod cometbft {

    pub mod version {
        pub mod v1 {
            include!("./proto/cometbft.version.v1.rs");
        }
    }
    pub mod crypto {
        pub mod v1 {
            include!("./proto/cometbft.crypto.v1.rs");
        }
    }

    pub mod p2p {
        pub mod v1 {
            include!("./proto/cometbft.p2p.v1.rs");
        }
    }

    pub mod privval {
        pub mod v1beta1 {
            include!("./proto/cometbft.privval.v1beta1.rs");
        }
        pub mod v1beta2 {
            include!("./proto/cometbft.privval.v1beta2.rs");
        }
        pub mod v1 {
            include!("./proto/cometbft.privval.v1.rs");
        }
    }

    pub mod types {
        pub mod v1beta1 {
            include!("./proto/cometbft.types.v1beta1.rs");
        }
        pub mod v1beta2 {
            include!("./proto/cometbft.types.v1beta2.rs");
        }
        pub mod v1 {
            include!("./proto/cometbft.types.v1.rs");
        }
    }
}

pub mod v0_34 {
    pub use crate::proto::cometbft::crypto::v1 as crypto;
    pub use crate::proto::cometbft::p2p::v1 as p2p;
    pub use crate::proto::cometbft::privval::v1beta1 as privval;
    pub use crate::proto::cometbft::types::v1beta1 as types;
}

pub mod v0_37 {
    pub use crate::proto::cometbft::crypto::v1 as crypto;
    pub use crate::proto::cometbft::p2p::v1 as p2p;
    pub use crate::proto::cometbft::privval::v1beta1 as privval;
    pub use crate::proto::cometbft::types::v1beta2 as types;
}

pub mod v0_38 {
    pub use crate::proto::cometbft::crypto::v1 as crypto;
    pub use crate::proto::cometbft::p2p::v1 as p2p;
    pub use crate::proto::cometbft::privval::v1beta2 as privval;
    pub use crate::proto::cometbft::types::v1 as types;
}

pub mod v1 {
    pub use crate::proto::cometbft::crypto::v1 as crypto;
    pub use crate::proto::cometbft::p2p::v1 as p2p;
    pub use crate::proto::cometbft::privval::v1 as privval;
    pub use crate::proto::cometbft::types::v1 as types;
}
