#!/usr/bin/env python3


import mtvimages
import mtvtables
import os
from pprint import pprint
import sqlite3
import utils
from dotenv import load_dotenv

CWD = os.getcwd()

class Main:
    def __init__(self):
        load_dotenv()
        self.conn = sqlite3.connect(os.getenv("MTVT_DB_PATH"))
        self.cursor = self.conn.cursor()
        

    def main(self):
    
        try:
            mtvtables.CreateTables().create_tables()

            images = utils.img_walk_dirs(os.getenv("MTVT_POSTER_PATH"))
            mtvimages.ProcessImages(images, self.conn, self.cursor).process()

        except sqlite3.OperationalError as e:
            print(e)
        finally:
            self.conn.close()

if __name__ == "__main__":
    m = Main()
    m.main()
