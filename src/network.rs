use async_trait::async_trait;
use futures::prelude::*;
use libp2p::core::upgrade::{read_length_prefixed, write_length_prefixed, ProtocolName};
use libp2p::request_response::RequestResponseCodec;
use log::debug;
use std::io;

#[derive(Debug, Clone)]
pub struct BlockchainExchangeProtocol();

#[derive(Clone)]
pub struct BlockchainExchangeCodec();
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockchainRequest();
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockchainResponse(pub String);

impl ProtocolName for BlockchainExchangeProtocol {
    fn protocol_name(&self) -> &[u8] {
        "/blockchain-exchange/1".as_bytes()
    }
}

#[async_trait]
impl RequestResponseCodec for BlockchainExchangeCodec {
    type Protocol = BlockchainExchangeProtocol;
    type Request = BlockchainRequest;
    type Response = BlockchainResponse;

    async fn read_request<T>(
        &mut self,
        _: &BlockchainExchangeProtocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let cmd = read_length_prefixed(io, 100_000_000).await?;
        debug!("p2p::Client::read_request for BlockchainExchangeCodec read {:?} - this is a peer telling the client to send the block", String::from_utf8(cmd.clone()).unwrap());

        if cmd.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        //
        Ok(BlockchainRequest()) //TODO: can I just return OK from here with no type
    }

    //reads the peer block
    async fn read_response<T>(
        &mut self,
        _: &BlockchainExchangeProtocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let vec = read_length_prefixed(io, 100_000_000).await?;

        if vec.is_empty() {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        Ok(BlockchainResponse(String::from_utf8(vec).unwrap()))
    }

    async fn write_request<T>(
        &mut self,
        _: &BlockchainExchangeProtocol,
        io: &mut T,
        BlockchainRequest(): BlockchainRequest,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, "idle_metric").await?;

        io.close().await?;

        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &BlockchainExchangeProtocol,
        io: &mut T,
        BlockchainResponse(data): BlockchainResponse,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        write_length_prefixed(io, data).await?;
        io.close().await?;

        Ok(())
    }
}
