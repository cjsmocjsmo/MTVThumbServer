#!/usr/bin/env python3

class CreateTables:
    def __init__(self, conn, cursor):
        self.conn = conn
        self.cursor = cursor

    def create_tables(self):
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



        

        
