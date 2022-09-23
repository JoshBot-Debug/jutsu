# USAGE

## Get client info
Returns client's system info
#### Command:
	jutsu -i 192.168.1.1 --info
	OR
	jutsu -i 192.168.1.1-255 --info
	OR
	jutsu -i 192.168.1.1,4,8 --info
#### Returns:
	IpAddress, Hostname, Session, CPU, RAM


## Get client by user session
Searches a client for the specified username where a session is active.
#### Command:
	jutsu -i 192.168.1.1-255 -f <username> --info
#### Returns:
	IpAddress, Hostname, Session, CPU, RAM


## Deploy a client via SSH
Installs a client via ssh
#### Command:
	jutsu --deploy joshua@192.168.1.1-255
