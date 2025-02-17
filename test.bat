@echo off
setlocal enabledelayedexpansion

:: 获取当前日期并格式化
for /f "tokens=2 delims==" %%i in ('"wmic os get localdatetime /value"') do set datetime=%%i
set year=!datetime:~0,4!
set month=!datetime:~4,2!
set day=!datetime:~6,2!
set hour=!datetime:~8,2!
set minute=!datetime:~10,2!
set second=!datetime:~12,2!

:: 生成带有时间戳的文件名
set timestamp=%year%-%month%-%day%_%hour%-%minute%-%second%
set outfile=test_%timestamp%.exr

:: 执行你的命令
@REM .\build\Debug\pbrt.exe --spp 16 .\scene\contemporary-bathroom\contemporary-bathroom-test.pbrt --outfile %outfile%
@REM .\build\Debug\pbrt.exe --spp 4 .\scene\contemporary-bathroom\contemporary-bathroom-sppm.pbrt --outfile %outfile%
@REM .\build\Debug\pbrt.exe --spp 4 .\scene\contemporary-bathroom\contemporary-bathroom-test.pbrt --outfile %outfile%
@REM .\build\Debug\pbrt.exe --spp 10 ..\pbrt-v4-scenes-master\bistro\bistro_vespa-test.pbrt --outfile %outfile%
@REM .\build\Debug\pbrt.exe --spp 2 ..\pbrt-v4-scenes-master\pbrt-book\book-test.pbrt --outfile %outfile%
.\build\Debug\pbrt.exe --spp 4 E:\Data\BJTU\NJU1\4.Research\pbrt-v4-scenes-master\barcelona-pavilion\pavilion-day-largeframe.pbrt --outfile ./test_barcelona-pavilion_sppm.exr

endlocal