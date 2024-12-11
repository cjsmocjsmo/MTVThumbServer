#!/usr/bin/env python3

import os
# import subprocess
# import sqlite3

def get_arch():
    arch =  os.uname().machine
    if arch == "armv7l":
        return "32"
    elif arch == "arm64" or arch == "x86_64":
        return "64"
    
# def mtv_walk_dirs(directory):
#     medialist = []
#     for root, dirs, files in os.walk(directory):
#         for file in files:
#             fname = os.path.join(root, file)
#             ext = os.path.splitext(fname)[1]
#             if ext == ".mp4":
#                 medialist.append(fname)
#     return medialist

# def movie_count():
#     count = 0
#     for root, dirs, files in os.walk(os.getenv("MTV_MOVIES_PATH")):
#         for file in files:
#             fname = os.path.join(root, file)
#             ext = os.path.splitext(fname)[1]
#             if ext == ".mp4":
#                 count += 1
#     return count

# def tvshow_count():
#     count = 0
#     for root, dirs, files in os.walk(os.getenv("MTV_TV_PATH")):
#         for file in files:
#             fname = os.path.join(root, file)
#             ext = os.path.splitext(fname)[1]
#             if ext == ".mp4":
#                 count += 1
#     return count

# def movies_size_on_disk():
#     conn = sqlite3.connect('your_database.db')  # Replace 'your_database.db' with your actual database file
#     cursor = conn.cursor()

#     cursor.execute("SELECT Size FROM movies")
#     sizes = cursor.fetchall()
    
#     size_list = [int(size[0]) for size in sizes]
#     total_movie_size = sum(size_list)
    
#     conn.close()
    
#     total_movie_size_gb = total_movie_size / (1024 ** 3)  # Convert bytes to gigabytes
    
#     return total_movie_size_gb

# def tvshows_size_on_disk():
#     conn = sqlite3.connect('your_database.db')  # Replace 'your_database.db' with your actual database file
#     cursor = conn.cursor()

#     cursor.execute("SELECT Size FROM tvshows")
#     sizes = cursor.fetchall()
    
#     size_list = [int(size[0]) for size in sizes]
#     total_tvshow_size = sum(size_list)
    
#     conn.close()
    
#     total_tvshow_size_gb = total_tvshow_size / (1024 ** 3)  # Convert bytes to gigabytes
    
#     return total_tvshow_size_gb

def img_walk_dirs(dir):
    jpglist = []
    for root, dirs, files in os.walk(dir):
        for file in files:
            fname = os.path.join(root, file)
            ext = os.path.splitext(fname)[1]
            if ext == ".jpg":
                jpglist.append(fname)
    return jpglist

# def sqlite3_check():
#     sqlite3 = subprocess.run(["apt-cache", "policy", "sqlite3"])
#     if sqlite3.returncode == 0:
#         return True
#     else:
#         return False
    
# def vlc_check():
#     vlc = subprocess.run(["apt-cache", "policy", "vlc"])
#     if vlc.returncode == 0:
#         return True
#     else:
#         return False
    
# def python3_vlc_check():
#     pvlc = subprocess.run(["apt-cache", "policy", "python3-vlc"])
#     if pvlc.returncode == 0:
#         return True
#     else:
#         return False
    
# def python3_pil_check():
#     pil = subprocess.run(["apt-cache", "policy", "python3-pil"])
#     if pil.returncode == 0:
#         return True
#     else:
#         return False
    
# def python3_dotenv_check():
#     dot = subprocess.run(["apt-cache", "policy", "python3-dotenv"])
#     if dot.returncode == 0:
#         return True
#     else:
#         return False

# def python3_websockets_check():
#     ws = subprocess.run(["apt-cache", "policy", "python3-websockets"])
#     if ws.returncode == 0:
#         return True
#     else:
#         return False