@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "CLIENT_DIR=%SCRIPT_DIR%client"

where npm >nul 2>nul
if errorlevel 1 (
  echo Error: npm is not installed or not available in PATH.
  exit /b 1
)

if not exist "%CLIENT_DIR%" (
  echo Error: client directory not found: %CLIENT_DIR%
  exit /b 1
)

cd /d "%CLIENT_DIR%" || exit /b 1

if not exist "package.json" (
  echo Error: client package.json not found.
  exit /b 1
)

echo Ensuring frontend dependencies...
call npm install || exit /b 1

echo Starting frontend development server...
echo Frontend URL: http://127.0.0.1:5173
call npm run dev -- --host 127.0.0.1 --port 5173
