docker build -t mtvtserver:latest .;
docker run -d \
-v /usr/share/MTV/mtv.db:/usr/share/MTV/mtv.db \
-v /usr/share/MTV/thumbnails/:/usr/share/MTV/thumbnails/ \
-v /home/pimedia/PINAS/MTV/Posters/:/usr/share/MTV/Posters/ \
-p 8082:80 \
mtvtserver:latest