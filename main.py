#!/usr/bin/env python3

import mtvimages
import os
from pprint import pprint
import sqlite3

CWD = os.getcwd()
MTVT_DB_PATH = "/usr/share/MTV/mtv.db"
MTVT_THUMBNAIL_PATH = "/usr/share/MTV/thumbnails/"
MTVT_POSTER_PATH = "/home/pimedia/PINAS/MTV/Posters/"
MTVT_SERVER_ADDR = "http://10.0.4.41"
MTVT_RAW_ADDR = "10.0.4.41"
MTVT_SERVER_PORT = "8082"

class Main:
    def __init__(self):
        self.conn = sqlite3.connect(MTVT_DB_PATH)
        self.cursor = self.conn.cursor()

    def create_tables(self):
        try:
            self.cursor.execute("""CREATE TABLE IF NOT EXISTS movies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                MovieId TEXT NOT NULL,
                Path TEXT NOT NULL,
                Size TEXT NOT NULL,
                Name TEXT NOT NULL,
                ThumbPath TEXT NOT NULL,
                HttpThumbPath TEXT NOT NULL
            )""")
        except sqlite3.OperationalError as e:
            print(e)

    def img_walk_dirs(self, dir):
        jpglist = []
        for root, dirs, files in os.walk(dir):
            for file in files:
                fname = os.path.join(root, file)
                ext = os.path.splitext(fname)[1]
                if ext == ".jpg":
                    jpglist.append(fname)
        return jpglist
        
    def main(self):
        self.create_tables()

        images = self.img_walk_dirs(MTVT_POSTER_PATH)
        mtvimages.ProcessImages(
            images, 
            self.conn, 
            self.cursor,
            MTVT_DB_PATH,
            MTVT_THUMBNAIL_PATH,
            MTVT_POSTER_PATH,
            MTVT_SERVER_ADDR,
            MTVT_RAW_ADDR,
            MTVT_SERVER_PORT,
            ).process()



if __name__ == "__main__":
    m = Main()
    m.main()
