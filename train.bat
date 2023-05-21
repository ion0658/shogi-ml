@echo off
set GAMES=256
set EPOCHS=10

for /l %%i in (0, 1, %EPOCHS%) do (
    echo "EPOCH %%i"
    cargo run --release --bin train %GAMES%
    python .\py_src\train.py
)