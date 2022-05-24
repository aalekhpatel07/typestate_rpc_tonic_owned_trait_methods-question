use std::time::Duration;
use tonic::{Request, Response, Code, Status};
use typestate_rpc::{msg_rpc_server::MsgRpc, MsgRequest, MsgResponse};


#[derive(Debug, Default)]
pub struct A {
    pub field3: usize
}

#[derive(Debug, Default)]
pub struct B {
    pub field2: usize
}


pub struct C<S> {
    pub state: S,
    pub field1: usize,
}


// Imagine some state transition is defined
// with some special business logic.
impl From<C<B>> for C<A> {
    fn from(c: C<B>) -> Self {
        C {
            state: A { field3: 0 },
            field1: c.field1
        }
    }
}

#[tonic::async_trait]
impl MsgRpc for C<B>
{
    async fn msg(&self, request: Request<MsgRequest>) -> Result<Response<MsgResponse>, Status> {

        // Do some processing of the request.
        std::thread::sleep(Duration::from_secs(2));

        // Suppose we determine based on the result of the processing that `self` should transition to C<A>.
        // But we don't have ownership of self so we can't do it as is.

        // How do we get `C<A>::from(self)` to work here, in this kind of design where types represent
        // states of a state machine and RPCs can cause them to transition?

        // Here, &self is &C<B>. To simulate `&mut self` here, we can also put our mutable state
        // behind an Arc Mutex, like `Arc<Mutex<State>>>`. But iiuc, I don't want `&mut self`, I want `self` instead. 
        // Is there an idiomatic way to get to `self` here
        // instead? Or otherwise, is there some tonic_build config that generates 
        // trait definitions with owned methods instead?


        Err(Status::new(Code::Unimplemented, ""))

    }
}

fn main() {
    println!("Hello, world!");
}
