
use tokio;
use server::rtp_server::RtpServer;

use futures::prelude::*;

fn main() {
    let server = RtpServer::new();
    let server2 = RtpServer::new();
    let joined = server.join(server2).map( |_|() );

    tokio::run(joined);
}
