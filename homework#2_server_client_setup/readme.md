# Smarthouse lib via async server ( tokio )
## Usage
1. launch the server via command ``cargo run -p server_tokio``
2. launch client app, specifying a valid dev name and optional enable\disable CLI command.
### Examples
1. `` cargo run -p server_tokio --example client_example -- -d termometer_#0 -e true`` to enable termometer with a name termometer_#0
2. `` cargo run -p server_tokio --example client_example -- -d termometer_#0 `` to get information from termometer_#0
3. `` cargo run -p server_tokio --example client_example -- --h `` information 

## Notes
Termometer's and socket's data are being updated every 100 mili seconds.
