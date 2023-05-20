@echo off
set GAMES=128
set EPOCH=10

for /l %%i in (0, 1, EPOCH) do (
    echo "EPOCH %%i"
    cargo run --release --bin train %GAMES% %%i 4
    python .\py_src\train.py %%i
)