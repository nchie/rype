
use tokio;
use server::rtp_server::RtpServer;
use server::rtsp_server::{ RtspServer };

use futures::prelude::*;

fn main() {
    // let server = RtspServer::builder()
    //     .serve();
    // let server2 = RtpServer::new();
    // let joined = server.join(server2).map( |_|() );

    // tokio::run(joined);
}
