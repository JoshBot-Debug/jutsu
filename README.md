# JUTSU

## Get client info
Returns client's identity.
#### Command:
	jutsu -i 192.168.1.1		// Single client
	OR
	jutsu -i 192.168.1.1-254	// Range of clients
	OR
	jutsu -i 192.168.1.1,4,8	// Specific clients
#### Returns:
	IpAddress, Hostname, Session(s)

## Filter client(s) by matching user session.
Filter client(s) by the specified username where the user session is active.
You can pass a single client, multiple clients or a range
#### Command:
	jutsu -i 192.168.1.1-254 -u <username>
#### Returns:
	IpAddress, Hostname, Session(s)

## Install or uninstall a client via SSH
You can pass a single client, multiple clients or a range
#### Deploy
	jutsu --deploy-client joshua@192.168.1.1-254

#### Purge
	jutsu --purge-client joshua@192.168.1.1-254

## Build client
	cargo build --target x86_64-unknown-linux-musl --bin jutsu-client --release