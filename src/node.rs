#![allow(dead_code)]

use std::net::Ipv4Addr;


pub type Nodes = Vec<Node>;

#[derive(Clone, Debug)]
pub struct Node
{
    address: Ipv4Addr,
    port: u32,
}


impl Node
{
    pub fn new(address: Ipv4Addr, port: u32) -> Self
    {
        Self { address, port }
    }

    pub fn address(&self) -> &Ipv4Addr
    {
        &self.address
    }

    pub fn port(&self) -> u32
    {
        self.port
    }
}
