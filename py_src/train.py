import sqlite3
import numpy as np
import tensorflow as tf
import matplotlib.pyplot as plt
import os

BOARD_SIZE = 9
PIECE_TYPES = 14
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 2 * 2 * PIECE_TYPES
MODEL_DIR = "model/model"
EPOCHS = 5


def load_game_data():
    dbname = "db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()
    sql = "SELECT WINNER, RECORDS FROM KIFU"
    cur.execute(sql)
    game_data = cur.fetchall()
    cur.close()
    conn.close()
    x = []
    y = []
    black_win_count = 0
    game_count = 0
    for row in game_data:
        (winner, binary) = row
        array_1d = np.frombuffer(binary, dtype=np.uint8)
        record = array_1d.reshape(
            [
                int(len(array_1d) / BOARD_SQ_SIZE),
                BOARD_SIZE,
                BOARD_SIZE,
                2 * 2 * PIECE_TYPES,
            ]
        )
        if winner == 0:
            black_win_count += 1
        game_count += 1
        for board in record:
            x.append(board)
            y.append(winner)
    X = np.array(x)
    Y = np.array(y)
    return (X, Y)


def load_model():
    if os.path.exists(MODEL_DIR):
        model = tf.keras.models.load_model(MODEL_DIR)
        return model
    else:
        model = tf.keras.models.Sequential(
            [
                tf.keras.layers.Conv2D(
                    64,
                    (3, 3),
                    activation="relu",
                    input_shape=(BOARD_SIZE, BOARD_SIZE, 2 * 2 * PIECE_TYPES),
                    name="board_in",
                ),
                tf.keras.layers.MaxPooling2D((2, 2)),
                tf.keras.layers.Conv2D(128, (3, 3), activation="relu"),
                tf.keras.layers.Flatten(),
                tf.keras.layers.Dense(128, activation="relu"),
                tf.keras.layers.Dropout(rate=0.2),
                tf.keras.layers.Flatten(),
                tf.keras.layers.Dense(64, activation="relu"),
                tf.keras.layers.Dropout(rate=0.2),
                tf.keras.layers.Flatten(),
                tf.keras.layers.Dense(32, activation="relu"),
                tf.keras.layers.Dropout(rate=0.2),
                tf.keras.layers.Dense(2, activation="softmax", name="winner_out"),
            ]
        )
        model.compile(
            optimizer="adam",
            loss="sparse_categorical_crossentropy",
            metrics=["accuracy"],
        )
        return model


def show_graph(history):
    # グラフ描画(2画面)
    plt.figure(figsize=(16, 8))

    # epochごとのlossを表示
    plt.subplot(1, 2, 1)
    plt.plot(range(1, EPOCHS + 1), history.history["loss"], "-o")
    plt.plot(range(1, EPOCHS + 1), history.history["val_loss"], "-o")
    plt.title("loss_transition")
    plt.ylabel("loss")
    plt.xlabel("epoch")
    plt.grid()
    plt.legend(["loss", "val_loss"], loc="best")

    # epochごとのaccuracyを表示
    plt.subplot(1, 2, 2)
    plt.plot(range(1, EPOCHS + 1), history.history["accuracy"], "-o")
    plt.plot(range(1, EPOCHS + 1), history.history["val_accuracy"], "-o")
    plt.title("accuracy_transition")
    plt.ylabel("accuracy")
    plt.xlabel("epoch")
    plt.grid()
    plt.legend(["accuracy", "val_accuracy"], loc="best")

    # グラフ表示
    plt.show()


x, y = load_game_data()

model = load_model()
model.summary()

# 学習開始
history = model.fit(x, y, epochs=EPOCHS, validation_split=0.2)
model.save(MODEL_DIR)
# show_graph(history)

os.remove("db/data.db")
