use crate::error::HederaError;

// The string specified here must match the proto package name
pub mod services {
    tonic::include_proto!("proto");
}

pub mod sdk {
    tonic::include_proto!("sdk/proto");
}

pub mod mirror {
    tonic::include_proto!("mirror/com.hedera.mirror.api.proto");
}

pub mod streams {
    tonic::include_proto!("streams/proto");
}

pub trait ToProto<T> {
    fn to_proto(&self) -> Result<T, HederaError>;
}
