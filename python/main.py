 # xato-net-10-million-passwords.txtから指定のパスワード探す

import glob
import time
from pathlib import Path

password = input("Please input password.")
print(f'password: {password}')

# 計測開始(ナノ秒)
start_time = time.perf_counter_ns()
# 指定ディレクトリ内からファイル名を取得 
files = glob.glob("./xato-net-10-million-passwords.txt")
# ファイルを一つずつ処理
for file in files:
    try:
        with open(file) as f:
            # パスワードリストから１行ずつ取得してチェック
            for line in f:
                if line.strip() == password:
                    # 計測終了(ナノ秒)
                    end_time = time.perf_counter_ns()
                    print(f'✔ cracking')
                    # 経過時間を出力(ミリ秒)
                    elapsed_time = (end_time - start_time) / 1000000
                    print(f"{elapsed_time}ms")
                    continue
    # 'utf-8' codec can't decode
    except:
        print(f"error")
