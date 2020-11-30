FROM alpine:latest

RUN apk update && \
    apk add nodejs npm git nano && \
    git clone https://github.com/AssystDev/Assyst /home/assyst && \
    cd /home/assyst && \
    npm i && \
    npm i -g typescript && \
    cp config.template.json config.json && \
    mkdir /home/assyst/dist

CMD sh
