# 並列処理
# xato-net-10-million-passwords-dup.txtを2分割したsplit_000.txtとsplit_001.txtから指定のパスワード探す

import glob
import time
from pathlib import Path
import threading

password = input("Please input password.\n")
file1 = glob.glob("./split_000.txt")
file2 = glob.glob("./split_001.txt")

# split_000.txtの処理
def proc1():

    # 計測開始(ナノ秒)
    start_time = time.perf_counter_ns()
    # 何個目で当てたかのカウント
    count = 0

    # ファイルを一つずつ処理
    for file in file1:
        print(f'⠏cracking1') 
        try:
            with open(file) as f:
                # パスワードリストから１行ずつ取得してチェック
                for line in f:
                    if line.strip() == password:
                        # 計測終了(ナノ秒)
                        end_time = time.perf_counter_ns()
                        print(f'no.{count}') 
                        print(f'found {password}')
                        print(f'✔ cracking1')
                        # 経過時間を出力(ミリ秒)
                        elapsed_time = (end_time - start_time) / 1000000
                        print(f"{elapsed_time}ms")
                        continue
                    count += 1
        # 'utf-8' codec can't decode
        except:
            print(f"error")


# split_001.txtの処理
def proc2():

    # 計測開始(ナノ秒)
    start_time = time.perf_counter_ns()
    # 何個目で当てたかのカウント
    count = 0

    # ファイルを一つずつ処理
    for file in file2:
        print(f'⠏cracking2') 
        try:
            with open(file) as f:
                # パスワードリストから１行ずつ取得してチェック
                for line in f:
                    if line.strip() == password:
                        # 計測終了(ナノ秒)
                        end_time = time.perf_counter_ns()
                        print(f'no.{count}') 
                        print(f'found {password}')
                        print(f'✔ cracking2')
                        # 経過時間を出力(ミリ秒)
                        elapsed_time = (end_time - start_time) / 1000000
                        print(f"{elapsed_time}ms")
                        continue
                    count += 1
        # 'utf-8' codec can't decode
        except:
            print(f"error")

if __name__ == '__main__':
    th1 = threading.Thread(target=proc1)
    th2 = threading.Thread(target=proc2)
    th1.start()
    th2.start()
