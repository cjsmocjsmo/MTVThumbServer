#!/usr/bin/env python3

import hashlib
import os
from PIL import Image
import re
import subprocess
import sqlite3
from pprint import pprint

class ProcessImages:
    def __init__(self, imgs, conn, cursor,
                MTVT_DB_PATH,
                MTVT_THUMBNAIL_PATH,
                MTVT_POSTER_PATH,
                MTVT_SERVER_ADDR,
                MTVT_RAW_ADDR,
                MTVT_SERVER_PORT):
        self.conn = conn
        self.cursor = cursor
        self.imglist = imgs
        self.search = re.compile("\s\(")
        self.MTVT_DB_PATH = MTVT_DB_PATH
        self.MTVT_THUMBNAIL_PATH = MTVT_THUMBNAIL_PATH
        self.MTVT_POSTER_PATH = MTVT_POSTER_PATH
        self.MTVT_SERVER_ADDR = MTVT_SERVER_ADDR
        self.MTVT_RAW_ADDR = MTVT_RAW_ADDR
        self.MTVT_SERVER_PORT = MTVT_SERVER_PORT

    def thumb_dir_check(self):
        if not os.path.exists(self.MTVT_THUMBNAIL_PATH):
            subprocess.run(["mkdir", self.MTVT_THUMBNAIL_PATH])
            print(f"Created directory")

    def create_thumbnail(self, img):
        fname = os.path.split(img)[1]
        save_path = os.path.join(self.MTVT_THUMBNAIL_PATH, fname)

        thumb = Image.open(img)
        thumb.thumbnail((300, 300))
        thumb.save(save_path)
        return save_path

    def get_img_id(self, imgstr):
        encoded_string = imgstr.encode('utf-8')
        md5_hash = hashlib.md5()
        md5_hash.update(encoded_string)
        return md5_hash.hexdigest()

    def get_name(self, img):
        img = os.path.split(img)[1]
        match = re.search(self.search, img)
        if match:
            start = match.start()
            return img[:start]
        else:
            print("No match")

    def get_thumb_path(self, img):
        fname = os.path.split(img)[1]
        return os.path.join(self.MTVT_THUMBNAIL_PATH, fname)

    def get_http_thumb_path(self, img):
        fname = os.path.split(img)[1]
        return f"{self.MTVT_SERVER_ADDR}:{self.MTVT_SERVER_PORT}/{fname}"
    
    def process(self):
        self.thumb_dir_check()
        for idx, img in enumerate(self.imglist):
            thumb = self.create_thumbnail(img)
            media_info = {
                "ImgId": self.get_img_id(thumb),
                "Size": os.stat(thumb).st_size,
                "Name": self.get_name(img),
                "ThumbPath": self.get_thumb_path(thumb),
                "Path": thumb,
                "Idx": idx+1,
                "HttpThumbPath": self.get_http_thumb_path(thumb),
            }
            pprint(media_info)
            
            try:
                self.cursor.execute('''INSERT INTO images (ImgId, Path, ImgPath, Size, Name, ThumbPath, Idx, HttpThumbPath)
                            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                ''', (
                    media_info["ImgId"], 
                    media_info["Path"], 
                    media_info["Path"], 
                    media_info["Size"], 
                    media_info["Name"], 
                    media_info["ThumbPath"], 
                    media_info["Idx"], 
                    media_info["HttpThumbPath"]
                ))
                self.conn.commit()
                
            except sqlite3.IntegrityError as e:
                print(f'Error: {e}')
            except sqlite3.OperationalError as e:
                print(f"Error: {e}")

