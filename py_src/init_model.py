import tensorflow as tf

BOARD_SIZE = 9
PIECE_TYPES = 14
BOARD_SQ_SIZE = BOARD_SIZE * BOARD_SIZE * 2 * 2 * PIECE_TYPES
MODEL_DIR = "model/model"
EPOCHS = 5


class myModel(tf.keras.Model):
    def __init__(self):
        super(myModel, self).__init__()
        self.conv1 = tf.keras.layers.Conv2D(
            64,
            (3, 3),
            activation="relu",
            input_shape=(BOARD_SIZE, BOARD_SIZE, 2 * 2 * PIECE_TYPES),
            name="board_in",
        )
        self.pool = tf.keras.layers.MaxPooling2D((2, 2))
        self.conv2 = tf.keras.layers.Conv2D(64, (3, 3), activation="relu")
        self.flatten = tf.keras.layers.Flatten()
        self.dense1 = tf.keras.layers.Dense(100, activation="relu")
        self.drop1 = tf.keras.layers.Dropout(rate=0.2)
        self.dense2 = tf.keras.layers.Dense(50, activation="relu")
        self.drop2 = tf.keras.layers.Dropout(rate=0.2)
        self.dense3 = tf.keras.layers.Dense(10, activation="relu")
        self.drop3 = tf.keras.layers.Dropout(rate=0.2)
        self.dense4 = tf.keras.layers.Dense(2, activation="softmax", name="winner_out")

    def call(self, x):
        x = self.conv1(x)
        x = self.pool(x)
        x = self.conv2(x)
        x = self.flatten(x)
        x = self.dense1(x)
        x = self.drop1(x)
        x = self.dense2(x)
        x = self.drop2(x)
        x = self.dense3(x)
        x = self.drop3(x)
        x = self.dense4(x)
        return x

    @tf.function
    def train_step(self, data):
        x, y_true = data
        with tf.GradientTape() as tape:
            # 予測
            y_pred = self(x, training=True)
            # train using gradients
            trainable_vars = self.trainable_variables
            # loss
            loss = self.compiled_loss(y_true, y_pred, regularization_losses=self.losses)
        # 勾配を用いた学習
        gradients = tape.gradient(loss, trainable_vars)
        self.optimizer.apply_gradients(
            (grad, var)
            for (grad, var) in zip(gradients, trainable_vars)
            if grad is not None
        )
        # update metrics
        self.compiled_metrics.update_state(y_true, y_pred)
        return {"loss": loss}
        # return {m.name: m.result() for m in self.metrics}

    def test_step(self, data):
        x, y_true = data
        # 予測
        y_pred = self(x, training=False)
        # loss
        self.compiled_loss(y_true, y_pred, regularization_losses=self.losses)
        # update metrics
        self.compiled_metrics.update_state(y_true, y_pred)
        return {m.name: m.result() for m in self.metrics}

    def predict_step(self, x):
        # 予測
        y_pred = self(x, training=False)
        return y_pred


model = myModel()
model.compile(
    optimizer="adam",
    loss="sparse_categorical_crossentropy",
    metrics=["acc"],
    run_eagerly=True,
)
input_shape = tf.TensorShape((None, BOARD_SIZE, BOARD_SIZE, 2 * 2 * PIECE_TYPES))
model.build(input_shape=input_shape)
model.summary()

x = tf.TensorSpec(shape=input_shape, dtype=tf.float32, name="x")
y = tf.TensorSpec(shape=(None, 1), dtype=tf.float32, name="y")
train = model.train_step.get_concrete_function((x, y))

board = tf.random.uniform((1, BOARD_SIZE, BOARD_SIZE, 2 * 2 * PIECE_TYPES))
predict = tf.function(model.predict_step).get_concrete_function(board)

model.save(MODEL_DIR, signatures={"serving_default": predict, "train": train})

# model = tf.keras.models.load_model(MODEL_DIR)
# model.summary()
