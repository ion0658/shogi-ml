import numpy as np
import tensorflow as tf
import sys
import sqlite3

BOARD_SIZE = 9
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 4

gen = 0
if len(sys.argv) > 1:
    gen = int(sys.argv[1])
file_name = "model/gen_{}".format(gen)

def load_game_data():
    dbname = "db/data.db"
    conn = sqlite3.connect(dbname)
    cur = conn.cursor()
    sql = "SELECT WINNER, RECORDS FROM KIFU WHERE GENERATION={} LIMIT 1".format(gen)
    cur.execute(sql)
    game_data = cur.fetchall()
    cur.close()
    conn.close()
    x = []
    y = []
    data_count = 0
    for row in game_data:
        (winner, binary) = row
        array_1d  = np.frombuffer(binary, dtype=np.uint8)
        record = array_1d.reshape([int(len(array_1d)/BOARD_SQ_SIZE), 4, BOARD_SIZE, BOARD_SIZE])
        data_count += int(len(array_1d)/BOARD_SQ_SIZE)
        for board in record:
            x.append(board)
            y.append(winner)
    print("data count: ", data_count)
    return (x, y)

model = tf.keras.models.load_model(file_name)
x, y = load_game_data()
x = x[10:11]
X = np.array(x)
X = X / X.max()
result = model.predict(X)
print(result)
    