import glob
import time

password = input(“パスワードを入力してください:“)
# 計測開始
start_time = time.process_time()
# 指定ディレクトリ内からファイル名を取得
files = glob.glob(“/Users/b/go/src/SecLists/Passwords/WiFi-WPA/*“)
for file in files:
    #with open(‘/Users/b/go/src/SecLists/Passwords/xato-net-10-million-passwords.txt’) as f:
    # ファイルを一つずつ処理
    with open(file) as f:
        # パスワードリストから１行ずつ取得してチェック
        for line in f:
            if line.strip() == password:
                print(f’found {line.strip()}’)
                # 計測終了
                end_time = time.process_time()
                # 経過時間を出力(秒)
                elapsed_time = end_time - start_time
                print(f”{elapsed_time}“)
                continue
