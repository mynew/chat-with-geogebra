@echo off
cd /d "%~dp0\..\next"
if errorlevel 1 (
    echo Failed to change directory to next
    exit /b 1
)

echo Installing dependencies...
call pnpm install
if errorlevel 1 (
    echo Failed to install dependencies
    exit /b 1
)

echo Building project...
call pnpm build
if errorlevel 1 (
    echo Failed to build project
    exit /b 1
)

echo Copying static assets...
xcopy /E /I /Y "%~dp0\..\next\.next\static" "%~dp0\..\next\.next\standalone\.next\static"

echo Copying standalone files into tauri...
xcopy /E /I /Y "%~dp0\..\next\.next\standalone" "%~dp0\..\tauri\standalone"

cd /d "%~dp0\..\tauri"

echo Installing tauri dependencies...
call pnpm install
if errorlevel 1 (
    echo Failed to install tauri dependencies
    exit /b 1
)

echo Building tauri...
call pnpm tauri build
if errorlevel 1 (
    echo Failed to build tauri
    exit /b 1
)

echo Returning to original directory...
cd /d "%~dp0..\"

echo Build completed successfully!