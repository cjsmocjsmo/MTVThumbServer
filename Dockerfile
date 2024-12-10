# Stage 1: Build stage
FROM debian:bookworm AS builder

RUN apt-get update && \
    apt-get dist-upgrade -y && \ 
    apt-get autoclean -y && \
    apt-get autoremove -y && \
    apt-get install python3-pil python3-dotenv -y && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir /usr/share/MTV && \
    mkdir /usr/share/MTV/thumbnails

WORKDIR /usr/share/MTV

COPY main.py /usr/share/MTV/
COPY mtvimages.py /usr/share/MTV/
COPY mtvtables.py /usr/share/MTV/
COPY utils.py /usr/share/MTV/
COPY .env /usr/share/MTV/

RUN python3 main.py

# Stage 2: Nginx stage
FROM nginx:bookworm

COPY --from=builder /usr/share/MTV/thumbnails /usr/share/nginx/html/

RUN rm /usr/share/nginx/html/index.html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]