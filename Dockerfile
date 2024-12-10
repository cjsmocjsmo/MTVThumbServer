# Stage 1: Build stage
FROM debian:bookworm AS builder

RUN apt-get update && \
    apt-get dist-upgrade -y && \ 
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    apt-get install python3-pil python3-dotenv -y

RUN mkdir /usr/share/MTV && \
    mkdir /usr/share/MTV/thumbnails

WORKDIR /usr/share/MTV

COPY main.py /usr/share/MTV/
COPY mtvimages.py /usr/share/MTV/
COPY mtvtables.py /usr/share/MTV/
COPY utils.py /usr/share/MTV/
COPY .env /usr/share/MTV/

RUN python3 main.py

FROM debian:bookworm

RUN apt-get update && \
    apt-get dist-upgrade -y && \ 
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/share/MTV/thumbnails /var/www/html/

EXPOSE 80

CMD ["apache2ctl", "-D", "FOREGROUND"]