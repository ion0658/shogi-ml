import sqlite3
import numpy as np
import tensorflow as tf
import matplotlib.pyplot as plt
import os
    
BOARD_SIZE = 9
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 4
EPOCHS = 10

def load_game_data():
    dbname = "db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()

    cur.execute("SELECT * FROM KIFU")
    game_data = cur.fetchall()
    cur.close()
    conn.close()
    x = []
    y = []
    data_count = 0
    for row in game_data:
        (_, winner, binary) = row
        array_1d  = np.frombuffer(binary, dtype=np.uint8)
        record = array_1d.reshape([int(len(array_1d)/BOARD_SQ_SIZE), 4, 9, 9])
        data_count += int(len(array_1d)/BOARD_SQ_SIZE)
        for board in record:
            x.append(board)
            y.append(winner)
    print(x[0][0])
    print(x[0][1])
    print(x[0][2])
    print(x[0][3])
    print("data count: ", data_count)
    return ( np.array(x) / 255.0, np.array(y) / 255.0)

def train():
    x, y = load_game_data()

    # 空のモデルを生成
    model = tf.keras.Sequential()
    # 入力データのサイズを設定、3はRGBの3色(PNGなら透過情報が付与されるので4?)
    model.add(tf.keras.Input(shape=(4, BOARD_SIZE, BOARD_SIZE)))
    # 平坦化層:入力データをチャンネルに関係なく全て1次元配列に変換
    model.add(tf.keras.layers.Flatten())
    # 全結合層:ユニット(パーセプトロン)数や活性化関数を設定
    model.add(tf.keras.layers.Dense(256, input_dim=10, activation="relu"))
    # ドロップアウト層: 過学習を防止する
    model.add(tf.keras.layers.Dropout(rate=0.2))
    # 出力のラベル数をセット(今回の構成では学習データを格納したフォルダ数になる)
    model.add(tf.keras.layers.Dense(2, activation="softmax"))
    # モデルの情報を出力
    model.summary()

    # モデルを構築
    model.compile(optimizer='adam',
                loss='sparse_categorical_crossentropy',
                metrics=['accuracy'])

    # 学習開始
    history = model.fit(x, y, epochs=EPOCHS, validation_split=0.2)
    model.save('model/model.h5')
    # グラフ描画(2画面)
    plt.figure(figsize=(16, 8))

    # epochごとのlossを表示
    plt.subplot(1, 2, 1)
    plt.plot(range(1, EPOCHS + 1), history.history['loss'], '-o')
    plt.plot(range(1, EPOCHS + 1), history.history['val_loss'], '-o')
    plt.title('loss_transition')
    plt.ylabel('loss')
    plt.xlabel('epoch')
    plt.grid()
    plt.legend(['loss', 'val_loss'], loc='best')

    # epochごとのaccuracyを表示
    plt.subplot(1, 2, 2)
    plt.plot(range(1, EPOCHS+1), history.history['accuracy'], '-o')
    plt.plot(range(1, EPOCHS+1), history.history['val_accuracy'], '-o')
    plt.title('accuracy_transition')
    plt.ylabel('accuracy')
    plt.xlabel('epoch')
    plt.grid()
    plt.legend(['accuracy', 'val_accuracy'], loc='best')

    # グラフ表示
    plt.show()
    

def check_dir():
    if not os.path.exists("model"):
        os.mkdir("model")
    if os.path.exists("model/model.h5"):
        os.remove("model/model.h5")
        
check_dir()
train()

