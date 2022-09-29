# USAGE

## Get client info
Returns client's partial system info and logged in users.
#### Command:
	jutsu -i 192.168.1.1
	OR
	jutsu -i 192.168.1.1-255
	OR
	jutsu -i 192.168.1.1,4,8
#### Returns:
	IpAddress, Hostname, Session(s), CPU, RAM

## Filter client(s) by matching user session.
Filter client(s) by the specified username where the user session is active.
#### Command:
	jutsu -i 192.168.1.1-255 -f <username>
#### Returns:
	IpAddress, Hostname, Session(s), CPU, RAM

## Deploy a client via SSH
Installs a client via ssh
#### Command:
	jutsu --deploy joshua@192.168.1.1-255
