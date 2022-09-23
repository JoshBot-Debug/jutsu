# USAGE

Command:
	jutsu -i 192.168.1.1 --info
	OR
	jutsu -i 192.168.1.1-255 --info
	OR
	jutsu -i 192.168.1.1,4,8 --info
Result:
	Returns info by default
	[RETURNS] IpAddress, Hostname, Session, CPU, RAM


Command:
	jutsu -i 192.168.1.1-255 -f <username> --info
Result: 
	Searches a client for the specified username where a session is active.
	[RETURNS] IpAddress, Hostname, Session, CPU, RAM
	
	
Command:
	jutsu --deploy joshua@192.168.1.1-255
Result:
	Deploy the client. SSH is required.