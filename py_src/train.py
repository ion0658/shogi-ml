import sqlite3
import numpy as np
import tensorflow as tf
import matplotlib.pyplot as plt
import os

BOARD_SIZE = 9
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 4
MODEL_DIR = "model/model"
EPOCHS = 5

physical_devices = tf.config.list_physical_devices("GPU")
if len(physical_devices) > 0:
    try:
        for dev in physical_devices:
            tf.config.experimental.set_memory_growth(dev, True)
            tf.config.set_logical_device_configuration(
                dev,
                [tf.config.LogicalDeviceConfiguration(memory_limit=4096)])
        logical_gpus = tf.config.list_logical_devices('GPU')
        print(len(physical_devices), "Physical GPUs,", len(logical_gpus), "Logical GPUs")
    except RuntimeError as e:
        # Virtual devices must be set before GPUs have been initialized
        print(e)

else:
    print("Not enough GPU hardware devices available")


def get_data_count():
    dbname = "db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()
    sql = "SELECT COUNT(*) FROM KIFU"
    cur.execute(sql)
    data_count = cur.fetchone()
    cur.close()
    conn.close()
    return data_count[0]

def load_game_data(start=0, count=1024):
    dbname = "db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()
    sql = "SELECT WINNER, RECORDS FROM KIFU WHERE ID>{} LIMIT {}".format(start, count)
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
        array_1d = np.frombuffer(binary, dtype=np.uint8)
        record = array_1d.reshape(
            [int(len(array_1d) / BOARD_SQ_SIZE), BOARD_SIZE, BOARD_SIZE, 4]
        )
        data_count += int(len(array_1d) / BOARD_SQ_SIZE)
        if winner == 0:
            black_win_count += 1
        game_count += 1
        for board in record:
            x.append(board)
            y.append(winner)
    print("data count: ", data_count)
    print("black winrate: {}%".format(black_win_count / game_count * 100))
    X = np.array(x)
    X = X / 14
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
                    input_shape=(BOARD_SIZE, BOARD_SIZE, 4),
                    name="board_in",
                ),
                tf.keras.layers.MaxPooling2D((2, 2)),
                tf.keras.layers.Conv2D(128, (3, 3), activation="relu"),
                tf.keras.layers.Flatten(),
                tf.keras.layers.Dense(64, activation="relu"),
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

all_data_count = get_data_count()
chunk_data_count = 1024
chunk = 1
chunk += all_data_count // chunk_data_count

for i in range(chunk):
    x, y = load_game_data(i*chunk_data_count, chunk_data_count)

    model = load_model()
    model.summary()

    # 学習開始
    history = model.fit(x, y, epochs=EPOCHS, validation_split=0.2)
    model.save(MODEL_DIR)
    # show_graph(history)
