# USAGE

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
#### Command:
	jutsu -i 192.168.1.1-254 -u <username>
#### Returns:
	IpAddress, Hostname, Session(s)

## Deploy a client via SSH
	jutsu --deploy-client joshua@192.168.1.1-254

## Deploy a client via SSH
	jutsu --purge-client joshua@192.168.1.1-254
