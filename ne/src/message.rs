use crate::NetworkResult;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io::{BufWriter, BufReader};
use std::net::TcpStream;
use std::io::Write;
use crossbeam_channel::{Sender, Receiver};

pub struct NetworkMessage<T> {
    result_sender: Option<Sender<NetworkResult<()>>>,
    data: T,
}

impl<T: Send + Serialize + DeserializeOwned> NetworkMessage<T> {
    pub fn new(data: T) -> (Self, Receiver<NetworkResult<()>>) {
        let (result_sender, result_receiver) = crossbeam_channel::bounded(1);
        (Self { result_sender: Some(result_sender), data }, result_receiver)
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn send(&self, writer: &mut BufWriter<TcpStream>) {
        let result = self.send_raw(writer);
        let _ = self.result_sender.as_ref().unwrap().send(result);
    }

    pub fn send_raw(&self, writer: &mut BufWriter<TcpStream>) -> NetworkResult<()> {
        serde_cbor::to_writer(writer.by_ref(), &self.data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn receive(reader: &mut BufReader<TcpStream>) -> NetworkResult<Self> {
        let data: T = serde_cbor::from_reader(reader)?;

        Ok(Self {
            result_sender: None,
            data,
        })
    }
}