import sqlite3
import numpy as np
import tensorflow as tf

BOARD_SIZE = 9
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 2
EPOCHS = 100

def load_game_data():
    dbname = "../db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()

    cur.execute("SELECT * FROM KIFU")
    game_data = cur.fetchall()
    cur.close()
    conn.close()
    x = []
    y = []
    count = 0
    for row in game_data:
        (_, winner, binary) = row
        array_1d  = np.frombuffer(binary, dtype=np.uint8)
        record = array_1d.reshape([int(len(array_1d)/BOARD_SQ_SIZE), 2, 9, 9])
        for board in record:
            x.append(board)
            y.append(winner)
            count += 1
    return (count, np.array(x) / 255.0, np.array(y) / 255.0)

count, x, y = load_game_data()
print("data count: ", count)

# 空のモデルを生成
model = tf.keras.Sequential()
# 入力データのサイズを設定、3はRGBの3色(PNGなら透過情報が付与されるので4?)
model.add(tf.keras.Input(shape=(2,BOARD_SIZE, BOARD_SIZE)))
# 平坦化層:入力データをチャンネルに関係なく全て1次元配列に変換
model.add(tf.keras.layers.Flatten())
# 全結合層:ユニット(パーセプトロン)数や活性化関数を設定
model.add(tf.keras.layers.Dense(256, input_dim=10, activation="relu"))
# ドロップアウト層: 過学習を防止する
model.add(tf.keras.layers.Dropout(rate=0.2))
# 出力のラベル数をセット(今回の構成では学習データを格納したフォルダ数になる)
model.add(tf.keras.layers.Dense(count, activation="softmax"))
# モデルの情報を出力
model.summary()

# モデルを構築
model.compile(optimizer='adam',
              loss='sparse_categorical_crossentropy',
              metrics=['accuracy'])

# 学習開始
history = model.fit(x, y, epochs=EPOCHS, validation_split=0.2)
