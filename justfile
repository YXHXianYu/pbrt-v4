# Set shell for Windows OSs:
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias b := build
alias r := run

# build and run
default: build run

# build the project
build:
    cmake --build build -j8

# run the built binary
run:
    # .\test.bat
    python .\data_maker.py