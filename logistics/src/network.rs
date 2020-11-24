use crate::graph::{Graph, Empty, NodeId};

struct Actor;

type Id = u32;

pub trait Resource {
  fn id(&self) -> u32;
}

struct Offer<R: Resource> {
  id: Id,
  supplier: NodeId,
  resource: R,
  count: u32,
  // Number of resources in offer that are tied up in allocations
  allocated: u32
}

struct Request<R: Resource> {
  id: Id,
  requester: NodeId,
  resource: R,
  priority: u32,
  count: u32,
  // Request is complete when delivered == count
  delivered: u32,
}

struct Order<R: Resource> {
  request_id: Id,
  supplier: NodeId,
  requester: NodeId,
  resource: R,
  count: u32
}

pub struct AuctionHouse<R: Resource> {
  offers: Vec<Offer<R>>,
  requests: Vec<Request<R>>,
  orders: Vec<Order<R>>
}

impl<R: Resource> AuctionHouse<R> {
  pub fn new() -> Self {
    AuctionHouse {
      offers: Vec::new(),
      requests: Vec::new(),
      orders: Vec::new()
    }
  }

  pub fn submit_offer(&mut self, supplier: NodeId, resource: R, count: u32) -> Id {
    unimplemented!()
  }

  pub fn submit_request(&mut self, requester: NodeId, resource: R, count: u32) -> Id {
    unimplemented!()
  }
}

