@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "SERVER_DIR=%SCRIPT_DIR%server"

where cargo >nul 2>nul
if errorlevel 1 (
  echo Error: cargo is not installed or not available in PATH.
  echo Install Rust from https://www.rust-lang.org/tools/install before starting the backend.
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

echo Starting Rust backend development server...
call cargo run
