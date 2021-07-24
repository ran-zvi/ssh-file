# SSH-FILE
CLI tool which allows ssh communication through files.

## Usage
```bash
ssh-file -f ~/endpoint -u <username> --ip-address <ip-address> &
```

Opens a new ssh connection to the provided ip address, which reads it's input commands from the file `~/endpoint`

Writing to the file will execute it's contents line by line on the ssh server and print them to stdout on the client, e.g:

```bash
echo "uname \n pwd" > ~/endpoint

> Linux
> /root
```

The file content is deleted immediately after the commands are executed.

On `SIGINT` the connections is closed and the file is deleted.