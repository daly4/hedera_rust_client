use crate::error::HederaError;
use crate::proto::services::{
    consensus_service_client::ConsensusServiceClient, crypto_service_client::CryptoServiceClient,
    file_service_client::FileServiceClient, freeze_service_client::FreezeServiceClient,
    network_service_client::NetworkServiceClient, schedule_service_client::ScheduleServiceClient,
    smart_contract_service_client::SmartContractServiceClient,
    token_service_client::TokenServiceClient,
};
use http::{uri::Authority, Uri};
use tonic::transport::{Certificate, Channel as TonicChannel, ClientTlsConfig};

#[derive(Debug, Clone)]
pub struct Channel {
    channel: TonicChannel,
}

impl Channel {
    pub fn new(channel: TonicChannel) -> Channel {
        Channel { channel }
    }

    fn build_uri(authority: &str) -> Result<Uri, HederaError> {
        let authority: Authority = authority.parse()?;
        let uri = Uri::builder()
            .scheme("https")
            .authority(authority)
            .path_and_query("/")
            .build()?;
        Ok(uri)
    }

    pub fn tonic_channel(authority: &str) -> Result<TonicChannel, HederaError> {
        let uri = Channel::build_uri(authority)?;
        Ok(TonicChannel::builder(uri).connect_lazy())
    }

    pub fn from_authority(authority: &str) -> Result<Channel, HederaError> {
        let tonic_channel = Self::tonic_channel(authority)?;
        Ok(Channel::new(tonic_channel))
    }

    pub fn from_authority_tls(authority: &str, cert: &[u8]) -> Result<Channel, HederaError> {
        let uri = Channel::build_uri(authority)?;
        let ca = Certificate::from_pem(cert);
        let tls = ClientTlsConfig::new().ca_certificate(ca);
        let tonic_channel = TonicChannel::builder(uri).tls_config(tls)?.connect_lazy();
        Ok(Channel::new(tonic_channel))
    }

    pub fn crypto(&self) -> CryptoServiceClient<TonicChannel> {
        CryptoServiceClient::new(self.channel.clone())
    }

    pub fn file(&self) -> FileServiceClient<TonicChannel> {
        FileServiceClient::new(self.channel.clone())
    }

    pub fn contract(&self) -> SmartContractServiceClient<TonicChannel> {
        SmartContractServiceClient::new(self.channel.clone())
    }

    pub fn topic(&mut self) -> ConsensusServiceClient<TonicChannel> {
        ConsensusServiceClient::new(self.channel.clone())
    }

    pub fn freeze(&mut self) -> FreezeServiceClient<TonicChannel> {
        FreezeServiceClient::new(self.channel.clone())
    }

    pub fn network(&mut self) -> NetworkServiceClient<TonicChannel> {
        NetworkServiceClient::new(self.channel.clone())
    }

    pub fn token(&mut self) -> TokenServiceClient<TonicChannel> {
        TokenServiceClient::new(self.channel.clone())
    }

    pub fn schedule(&mut self) -> ScheduleServiceClient<TonicChannel> {
        ScheduleServiceClient::new(self.channel.clone())
    }
}
