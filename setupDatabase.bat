@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "MIGRATION_FILE=%SCRIPT_DIR%server\migrations\20260605000000_create_users.sql"

where docker >nul 2>nul
if errorlevel 1 (
  echo Error: docker is not installed or not available in PATH.
  echo Install Docker Desktop, then rerun this script.
  exit /b 1
)

if not exist "%SCRIPT_DIR%.env" if exist "%SCRIPT_DIR%.env.example" (
  copy "%SCRIPT_DIR%.env.example" "%SCRIPT_DIR%.env" >nul
  echo Created .env from .env.example.
)

if not exist "%SCRIPT_DIR%server\.env" if exist "%SCRIPT_DIR%server\.env.example" (
  copy "%SCRIPT_DIR%server\.env.example" "%SCRIPT_DIR%server\.env" >nul
  echo Created server\.env from server\.env.example.
)

if exist "%SCRIPT_DIR%.env" (
  for /f "usebackq eol=# tokens=1,* delims==" %%A in ("%SCRIPT_DIR%.env") do (
    if not "%%A"=="" set "%%A=%%B"
  )
)

if "%POSTGRES_DB%"=="" set "POSTGRES_DB=nonewhite_site"
if "%POSTGRES_USER%"=="" set "POSTGRES_USER=nonewhite_user"

cd /d "%SCRIPT_DIR%" || exit /b 1
docker compose up -d postgres
if errorlevel 1 exit /b 1

echo Waiting for PostgreSQL to become ready...
for /l %%I in (1,1,30) do (
  docker compose exec -T postgres pg_isready -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" >nul 2>nul
  if not errorlevel 1 goto postgres_ready
  timeout /t 2 /nobreak >nul
)

echo Error: PostgreSQL did not become ready in time.
exit /b 1

:postgres_ready
type "%MIGRATION_FILE%" | docker compose exec -T postgres psql -U "%POSTGRES_USER%" -d "%POSTGRES_DB%"
if errorlevel 1 exit /b 1

echo Database migration applied: %MIGRATION_FILE%
