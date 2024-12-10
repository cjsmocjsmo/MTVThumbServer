docker build -t mtvtserver:latest \
-d \
-v /usr/share/MTV/mtv.db:/usr/share/MTV/mtv.db \
-v /usr/share/MTV/thumbnails/:/usr/share/MTV/thumbnails/ \
-v /usr/share/MTV/Posters/:/usr/share/MTV/Posters/ \
-p 8082:80 .
