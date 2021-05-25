# streams_demo
This is a small demo on Iota Streams (https://www.iota.org/solutions/streams) : a client written in rust that consume http rest services (thanks to https://github.com/nareph/channel-server) to create a stream, add messages, read the last message and messages history.
To launch:
  1- Install rust compiler: $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  2- Launch the server: https://github.com/nareph/channel-server
  3- Compile and run! $ cargo run
