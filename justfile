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
    # .\build\Debug\pbrt.exe --spp 4 ..\pbrt-v4-scenes-master\contemporary-bathroom\contemporary-bathroom-test.pbrt --outfile test.exr
    .\test.bat