#!/usr/bin/env python3

import os
import sqlite3

class CreateTables:
    def __init__(self):
        self.conn = sqlite3.connect(os.getenv("MTVT_DB_PATH"))
        self.cursor = self.conn.cursor()

    def create_tables(self):
        # self.cursor.execute("""CREATE TABLE IF NOT EXISTS movies (
        #     id INTEGER PRIMARY KEY AUTOINCREMENT,
        #     Name TEXT NOT NULL,
        #     Year TEXT NOT NULL,
        #     PosterAddr TEXT NOT NULL,
        #     Size TEXT NOT NULL,
        #     Path TEXT NOT NULL,
        #     Idx TEXT NOT NULL,
        #     MovId TEXT NOT NULL UNIQUE,
        #     Catagory TEXT NOT NULL,
        #     HttpThumbPath TEXT NOT NULL
        # )""")

        # self.cursor.execute("""CREATE TABLE IF NOT EXISTS tvshows (
        #     id INTEGER PRIMARY KEY AUTOINCREMENT,
        #     TvId TEXT NOT NULL UNIQUE,
        #     Size TEXT NOT NULL,
        #     Catagory TEXT NOT NULL,
        #     Name TEXT NOT NULL,
        #     Season TEXT NOT NULL,
        #     Episode TEXT NOT NULL,
        #     Path TEXT NOT NULL,
        #     Idx TEXT NOT NULL
        #  )""")
        
        self.cursor.execute("""CREATE TABLE IF NOT EXISTS images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ImgId TEXT NOT NULL,
            Path TEXT NOT NULL,
            ImgPath TEXT NOT NULL,
            Size TEXT NOT NULL,
            Name TEXT NOT NULL,
            ThumbPath TEXT NOT NULL,
            Idx INTEGER NOT NULL,
            HttpThumbPath TEXT NOT NULL
         )""")



        

        
