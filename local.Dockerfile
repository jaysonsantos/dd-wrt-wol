FROM ubuntu:20.04
COPY bin/dd-wrt-wol-api /dd-wrt-wol-api
CMD [ "sh", "-exc", "exec /dd-wrt-wol-api $(echo \"$HOSTS_CONFIG\" | tr ';' '\n' | while read i; do echo \"-h$i\"; done)" ]
