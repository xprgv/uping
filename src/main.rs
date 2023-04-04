use clap::Parser;
use utils::parse_target_addr;

mod client;
mod packet;
mod server;
pub mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 's', long = "server", help = "run in a server mode")]
    server: bool,

    #[arg(
        short = 'c',
        long = "count",
        default_value = "0",
        help = "stop after sending 'count' udp ping requests"
    )]
    count: u32,

    #[arg(
        short = 'i',
        long = "interval",
        default_value = "1s",
        help = "interval between packets"
    )]
    interval: String,

    address: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let interval = match parse_duration::parse(&args.interval) {
        Ok(interval) => interval,
        Err(e) => {
            eprintln!("failed to parse interval: {}", e);
            std::process::exit(1);
        }
    };

    let (socket_addr, dns_name) = parse_target_addr(&args.address).unwrap();

    if args.server {
        let server = server::Server::default();
        server.listen(socket_addr).await.unwrap();
    } else {
        let client = client::Client::default();
        client
            .ping(&socket_addr, &dns_name, args.count, interval)
            .await
            .unwrap();
    }
}
