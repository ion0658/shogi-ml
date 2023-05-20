@echo off
set GAMES=128
set START_EPOCK=10
set EPOCHS=10
set /a FIN_EPOCH=%START_EPOCK%+EPOCHS

for /l %%i in (%START_EPOCK%, 1, %FIN_EPOCH%) do (
    echo "EPOCH %%i"
    cargo run --release --bin train %GAMES% %%i 4
    python .\py_src\train.py %%i
)