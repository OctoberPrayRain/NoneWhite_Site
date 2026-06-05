@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "SERVER_DIR=%SCRIPT_DIR%server"

where npm >nul 2>nul
if errorlevel 1 (
  echo Error: npm is not installed or not available in PATH.
  exit /b 1
)

if not exist "%SERVER_DIR%" (
  echo Error: server directory not found: %SERVER_DIR%
  exit /b 1
)

cd /d "%SERVER_DIR%" || exit /b 1

if not exist ".env" if exist ".env.example" (
  copy ".env.example" ".env" >nul
  echo Created server\.env from server\.env.example.
)

if not exist "node_modules" (
  echo Installing backend dependencies...
  call npm install || exit /b 1
)

echo Starting backend development server...
call npm run dev
