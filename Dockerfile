# Stage 1: Build stage
FROM debian:bookworm AS builder

RUN apt-get update && \
    apt-get dist-upgrade -y && \ 
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    apt-get install python3-pil python3-dotenv -y

RUN mkdir /usr/share/MTV && \
    mkdir /usr/share/MTV/thumbnails && \
    chmod 777 /usr/share/MTV/thumbnails

WORKDIR /usr/share/MTV

COPY main.py /usr/share/MTV/
COPY mtvimages.py /usr/share/MTV/
COPY mtvtables.py /usr/share/MTV/
COPY utils.py /usr/share/MTV/
COPY .env /usr/share/MTV/

RUN python3 /usr/share/MTV/main.py

FROM debian:bookworm

RUN apt-get update && \
    apt-get dist-upgrade -y && \ 
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    apt-get install apache2 -y && \
    rm -rf /var/lib/apt/lists/*

RUN rm /var/www/html/index.html

COPY --from=builder /usr/share/MTV/thumbnails/ /var/www/html/

EXPOSE 80

CMD ["apache2ctl", "-D", "FOREGROUND"]