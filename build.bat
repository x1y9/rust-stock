@setlocal enableDelayedExpansion
@REM set LANG for grep run correctly 
@set LANG=zh_CN.UTF-8

@if  "%1"=="" goto do_help
@if  "%1"=="clean" goto do_clean
@if  "%1"=="version" goto do_version
@if  "%1"=="debug" goto do_debug
@if  "%1"=="release" goto do_release
@if  "%1"=="publish" goto do_publish

:do_help
@echo build script for android project
@echo.
@echo Build:   build [debug^|release^|publish]
@echo Clean:   build clean 
@echo Version: build version [number]
@goto end

:do_debug
cargo run
@IF %ERRORLEVEL% NEQ 0 goto error_end
@call toast debug-run
@goto end

:do_release
cargo run --release
@IF %ERRORLEVEL% NEQ 0 goto error_end
@call toast release-run
@goto end

:do_publish
@echo check uncommit files...
@git diff-files --quiet
@IF %ERRORLEVEL% NEQ 0 goto error_end
cargo build --release
@IF %ERRORLEVEL% NEQ 0 goto error_end
for /f %%i in ('grep -m 1 -oP "name = ""\K([a-zA-Z0-9.]+)" Cargo.toml') do set PACKAGE=%%i
for /f %%i in ('grep -m 1 -oP "version = ""\K([0-9.]+)" Cargo.toml') do set VERSION=%%i
for /f %%i in ('git rev-parse --short HEAD') do set HASH=%%i
@echo publish to %PACKAGE%-%VERSION%-%HASH%.exe
copy target\release\%PACKAGE%.exe %PACKAGE%-%VERSION%-%HASH%.exe
@call toast publish-build
@goto end

:do_clean
cargo clean
@goto end

:do_version
@if "%2"=="" (
@grep -m 1 -oP "version = ""\K([0-9.]+)" Cargo.toml
) else (
sed -i -E "s/version = ""([0-9.]+)""$/version = ""%2""/" Cargo.toml
cargo build --release
@IF %ERRORLEVEL% NEQ 0 goto error_end
git commit -m "%2" -a
)
@goto end

:error_end
@call toast build-fail
@echo Oops... Something wrong!
@ver /ERROR >NUL 2>&1

:end