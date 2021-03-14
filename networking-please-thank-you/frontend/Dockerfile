FROM nginx:stable-alpine

COPY ./dh4096.pem /etc/nginx
COPY ./cert.crt /etc/nginx
COPY ./cert.key /etc/nginx

COPY ./nginx.conf /etc/nginx/nginx.conf
