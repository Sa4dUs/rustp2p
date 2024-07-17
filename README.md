# 📂RustP2P. Peer-to-peer file sharing software
One-day file sharing system made in Rust.

## What is Peer-to-Peer?

Peer-to-peer (P2P) is a decentralized communication model where each participant, or "peer," has equivalent capabilities and responsibilities. Unlike the traditional client-server model, P2P networks distribute workload and resources among all peers, allowing for direct and efficient data exchange.

**Key Characteristics:**
- **Decentralization:** No central server, making the network more resilient.
- **Scalability:** Easily scales as more peers join, increasing network capacity.
- **Resource Sharing:** Peers share resources like bandwidth and storage.
- **Redundancy and Reliability:** Data is often replicated, improving availability.
- **Direct Communication:** Peers communicate directly, reducing latency.

**Common Uses:**
- File sharing (e.g., BitTorrent)
- Cryptocurrencies (e.g., Bitcoin)
- Content delivery networks (CDNs)
- Direct communication (e.g., VoIP services)

**Advantages:**
- **Resilience:** Resistant to single points of failure.
- **Cost-Effectiveness:** Leverages existing resources.
- **Enhanced Privacy:** Direct communication can enhance privacy.

**Challenges:**
- **Security:** Vulnerable to certain types of attacks.
- **Management:** Difficult to manage without central control.
- **Inconsistency:** Dynamic nature can lead to data integrity challenges.

## Features
- **Decentralized File Sharing**: Share files directly between peers without the need for a central server.
- **Cross-Platform**: Works on any platform that supports Rust.
- **Easy to Use**: Simple command-line interface for initiating and managing file transfers.
- **Fast and Efficient**: Built with Rust's performance and safety guarantees.

## Prerequisites
- Rust (stable) installed. If not, download and install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation
Clone the repository:
``` bash
git clone https://github.com/sa4dus/rustp2p.git
cd rustp2p
```
## Usage
To start listening in order to receive a file, use the `listen` subcommand:
```bash
cargo run --release -- listen
```  
The listening address will be displayed so you can send the data to the proper port.

Send files using the `send` subcommand:
```bash
cargo run --release -- send --address <IP_ADDRESS:PORT> --file <FILE_PATH>
```  

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## License
This project is licensed under the MIT License - see the LICENSE file for details.