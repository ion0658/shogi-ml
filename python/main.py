import numpy as np
from keras.models import Sequential
from keras.layers import Conv2D, MaxPooling2D, Flatten, Dense

def load_game_data():
    # 勝敗データは 1:先手勝利 -1:後手勝利
    # 盤面データは、0:空、正:先手 負:後手
    # 盤面サイズは 9x9×2の３次元配列
    # game_data としては、上記盤面データの連続で、最後に勝敗の記録を付ける
    # ダミーを作成する
    game_data = np.zeros((100, 163))
    
    return game_data
    
# データの読み込み
# 盤面データと勝敗の記録を取得する
game_data = load_game_data()
print(game_data)
# データの前処理
X = np.array(game_data[:, :-1])  # 盤面データ
y = np.array(game_data[:, -1])   # 勝敗の記録
print(X)
print(y)
print(X.shape[0])
# 盤面データの形状を変更する
X = X.reshape(X.shape[0], 9, 9, 2)  # (サンプル数, 9, 9, 2)

# モデルの設計
model = Sequential()
model.add(Conv2D(32, (3, 3), activation='relu', input_shape=(9, 9, 2)))
model.add(MaxPooling2D((2, 2)))
model.add(Flatten())
model.add(Dense(64, activation='relu'))
model.add(Dense(1, activation='sigmoid'))

# モデルのコンパイル
model.compile(optimizer='adam', loss='binary_crossentropy', metrics=['accuracy'])

# 学習
model.fit(X, y, epochs=10, batch_size=32, validation_split=0.2)

X_test = np.array([[
    [[1, 1], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [-1, 1]]
    ]])

y_test = np.array([-1])
# 評価
test_loss, test_acc = model.evaluate(X_test, y_test)
print('Test accuracy:', test_acc)

