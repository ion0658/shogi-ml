import sqlite3
import numpy as np
import tensorflow as tf
import matplotlib.pyplot as plt
import os
import sys
import shutil

BOARD_SIZE = 9
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 4

gen = 0
if len(sys.argv) > 1:
    gen = int(sys.argv[1])
file_name = "model/gen_{}".format(gen)
epochs = 5
if len(sys.argv) > 2:
    epochs = int(sys.argv[2])

def load_game_data():
    dbname = "db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()
    sql = "SELECT WINNER, RECORDS FROM KIFU WHERE GENERATION={}".format(gen)
    cur.execute(sql)
    game_data = cur.fetchall()
    cur.close()
    conn.close()
    x = []
    y = []
    black_win_count = 0
    game_count = 0
    data_count = 0
    for row in game_data:
        (winner, binary) = row
        array_1d  = np.frombuffer(binary, dtype=np.uint8)
        record = array_1d.reshape([int(len(array_1d)/BOARD_SQ_SIZE), 4, BOARD_SIZE, BOARD_SIZE])
        data_count += int(len(array_1d)/BOARD_SQ_SIZE)
        if winner == 0:
            black_win_count += 1
        game_count += 1
        for board in record:
            x.append(board)
            y.append(winner)
    print("data count: ", data_count)
    print("black winrate: {}%".format(black_win_count / game_count * 100))
    X = np.array(x)
    X = X / X.max()
    Y = np.array(y)
    return (X, Y)

def show_graph(history):
        # グラフ描画(2画面)
    plt.figure(figsize=(16, 8))

    # epochごとのlossを表示
    plt.subplot(1, 2, 1)
    plt.plot(range(1, epochs + 1), history.history['loss'], '-o')
    plt.plot(range(1, epochs + 1), history.history['val_loss'], '-o')
    plt.title('loss_transition')
    plt.ylabel('loss')
    plt.xlabel('epoch')
    plt.grid()
    plt.legend(['loss', 'val_loss'], loc='best')

    # epochごとのaccuracyを表示
    plt.subplot(1, 2, 2)
    plt.plot(range(1, epochs+1), history.history['accuracy'], '-o')
    plt.plot(range(1, epochs+1), history.history['val_accuracy'], '-o')
    plt.title('accuracy_transition')
    plt.ylabel('accuracy')
    plt.xlabel('epoch')
    plt.grid()
    plt.legend(['accuracy', 'val_accuracy'], loc='best')

    # グラフ表示
    plt.show()

def train():
    x, y = load_game_data()

    # 4. ニューラルネットワークモデルの定義
    model = tf.keras.models.Sequential([
        tf.keras.layers.Conv2D(64, (3, 3), activation='relu', input_shape=(4, BOARD_SIZE, BOARD_SIZE), name="board_in"),
        tf.keras.layers.Flatten(),
        tf.keras.layers.Dense(256, activation='relu'),
        #tf.keras.layers.Dropout(rate=0.2),
        tf.keras.layers.Dense(2, activation='softmax', name="winner_out")
    ])
    model.summary()


    # モデルを構築
    model.compile(optimizer='adam', loss='sparse_categorical_crossentropy', metrics=['accuracy'])

    # 学習開始
    history = model.fit(x, y, epochs=epochs, validation_split=0.2)
    model.save(file_name)
    #show_graph(history)
    

def check_dir():
    if not os.path.exists("model"):
        os.mkdir("model")
    if os.path.exists(file_name):
        shutil.rmtree(file_name)

check_dir()
train()

