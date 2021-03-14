FROM alpine:latest

RUN apk add --no-cache --update \
    samba-common-tools \
    samba-client \
    samba-server

COPY smb.conf /etc/samba/smb.conf

EXPOSE 137/udp 138/udp 139/tcp 445/tcp


CMD ["smbd", "--no-process-group", "--foreground", "--log-stdout"]

