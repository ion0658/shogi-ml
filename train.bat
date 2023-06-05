@echo off
set GAMES=1
set EPOCHS=100

for /l %%i in (0, 1, %EPOCHS%) do (
    echo "EPOCH %%i"
    .\target\release\train.exe %GAMES%
    python .\py_src\train.py
)