# dd-wrt-wol
Send Wake On Lan from your dd-wrt router without usage of port fowarding.
It is designed to run the cli on a dd-wrt but it can simply run in any platform that rust can compile to and there is a standard libary available.

## Usage
First you need to have the api runing somewhere like on a internet acessible server or a PaaS (like Heroku).
To run the api you can simply call `dd-wrt-lol-api --hosts 'name=desired-computer-name,mac_address=FF:FF:FF:FF:FF:FF,broadcast_ip=192.168.1.255'` and it should start listening on the port 8089.

## Running the cli
The second step is to run the cli.

### On dd-wrt
You can enable ssh and copy the binary to /jffs folder with `make copy-release` and after that configure it to run the cli on the background:

![Example](https://raw.githubusercontent.com/jaysonsantos/dd-wrt-wol/master/.docs/command.jpg)
```
/jffs/dd-wrt-wol-cli --machine-name=desired-computer-name -u http://your-server:8089 &
```
