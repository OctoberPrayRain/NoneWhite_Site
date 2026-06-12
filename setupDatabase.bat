@echo off
setlocal EnableExtensions

set "SCRIPT_DIR=%~dp0"
set "MIGRATIONS_DIR=%SCRIPT_DIR%server\migrations"
set "SEEDS_DIR=%SCRIPT_DIR%server\seeds"

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

if "%DB_SETUP_DRIVER%"=="" set "DB_SETUP_DRIVER=auto"
if "%POSTGRES_HOST%"=="" set "POSTGRES_HOST=localhost"
if "%POSTGRES_PORT%"=="" set "POSTGRES_PORT=5432"
if "%POSTGRES_DB%"=="" set "POSTGRES_DB=nonewhite_site"
if "%POSTGRES_USER%"=="" set "POSTGRES_USER=nonewhite_user"
if "%POSTGRES_PASSWORD%"=="" set "POSTGRES_PASSWORD=nonewhite_password"

if /I not "%DB_SETUP_DRIVER%"=="auto" if /I not "%DB_SETUP_DRIVER%"=="local" if /I not "%DB_SETUP_DRIVER%"=="docker" (
  echo Error: DB_SETUP_DRIVER must be one of: auto, local, docker.
  exit /b 1
)

call :command_exists psql PSQL_AVAILABLE
call :command_exists pg_isready PG_ISREADY_AVAILABLE
call :command_exists wsl.exe WSL_AVAILABLE
call :command_exists docker DOCKER_AVAILABLE
call :command_exists docker-compose DOCKER_COMPOSE_AVAILABLE

set "DB_RUNNER="
set "DOCKER_MODE="

if /I "%DB_SETUP_DRIVER%"=="local" (
  call :require_local_psql_tools || exit /b 1
  set "DB_RUNNER=local"
)

if /I "%DB_SETUP_DRIVER%"=="docker" (
  if not "%DOCKER_AVAILABLE%"=="1" (
    echo Error: docker is not installed or not available in PATH.
    exit /b 1
  )
  set "DB_RUNNER=docker"
)

if not defined DB_RUNNER (
  if "%PSQL_AVAILABLE%"=="1" if "%PG_ISREADY_AVAILABLE%"=="1" (
    call :database_connection_ready
    if not errorlevel 1 set "DB_RUNNER=local"
  )
)

if not defined DB_RUNNER (
  if /I "%DB_SETUP_DRIVER%"=="auto" if "%PSQL_AVAILABLE%"=="1" if "%PG_ISREADY_AVAILABLE%"=="1" if "%WSL_AVAILABLE%"=="1" (
    set "DB_RUNNER=wsl"
  )
)

if not defined DB_RUNNER (
  if "%DOCKER_AVAILABLE%"=="1" set "DB_RUNNER=docker"
)

if not defined DB_RUNNER (
  echo Error: could not find a usable PostgreSQL setup path.
  echo Install Docker Desktop, or expose a local PostgreSQL service with psql and pg_isready in PATH.
  echo On Windows you can also use the WSL fallback by keeping DB_SETUP_DRIVER=auto and optionally setting WSL_DB_DISTRO in .env.
  exit /b 1
)

if /I "%DB_RUNNER%"=="docker" (
  docker compose version >nul 2>nul
  if not errorlevel 1 (
    set "DOCKER_MODE=compose-plugin"
  ) else if "%DOCKER_COMPOSE_AVAILABLE%"=="1" (
    set "DOCKER_MODE=compose-standalone"
  ) else (
    echo Error: docker is available, but neither docker compose nor docker-compose is available.
    exit /b 1
  )
)

cd /d "%SCRIPT_DIR%" || exit /b 1

call :start_postgres
if errorlevel 1 exit /b 1

call :wait_for_postgres
if errorlevel 1 exit /b 1

if /I not "%DB_RUNNER%"=="docker" (
  call :verify_database_connection
  if errorlevel 1 exit /b 1
)

for %%F in ("%MIGRATIONS_DIR%\*.sql") do (
  if exist "%%~fF" (
    if /I "%DB_RUNNER%"=="docker" (
      if /I "%DOCKER_MODE%"=="compose-plugin" (
        type "%%~fF" | docker compose exec -T postgres psql -v ON_ERROR_STOP=1 -U "%POSTGRES_USER%" -d "%POSTGRES_DB%"
      ) else (
        type "%%~fF" | docker-compose exec -T postgres psql -v ON_ERROR_STOP=1 -U "%POSTGRES_USER%" -d "%POSTGRES_DB%"
      )
    ) else (
      set "PGPASSWORD=%POSTGRES_PASSWORD%"
      psql -v ON_ERROR_STOP=1 -h "%POSTGRES_HOST%" -p "%POSTGRES_PORT%" -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" -f "%%~fF"
    )
    if errorlevel 1 exit /b 1
    echo Database migration applied: %%~fF
  )
)

for %%F in ("%SEEDS_DIR%\dev_*.sql") do (
  if exist "%%~fF" (
    if /I "%DB_RUNNER%"=="docker" (
      if /I "%DOCKER_MODE%"=="compose-plugin" (
        type "%%~fF" | docker compose exec -T postgres psql -v ON_ERROR_STOP=1 -U "%POSTGRES_USER%" -d "%POSTGRES_DB%"
      ) else (
        type "%%~fF" | docker-compose exec -T postgres psql -v ON_ERROR_STOP=1 -U "%POSTGRES_USER%" -d "%POSTGRES_DB%"
      )
    ) else (
      set "PGPASSWORD=%POSTGRES_PASSWORD%"
      psql -v ON_ERROR_STOP=1 -h "%POSTGRES_HOST%" -p "%POSTGRES_PORT%" -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" -f "%%~fF"
    )
    if errorlevel 1 exit /b 1
    echo Development seed applied: %%~fF
  )
)

echo Database bootstrap completed using %DB_RUNNER%.
exit /b 0

:command_exists
where %~1 >nul 2>nul
if errorlevel 1 (
  set "%~2=0"
) else (
  set "%~2=1"
)
exit /b 0

:require_local_psql_tools
if not "%PSQL_AVAILABLE%"=="1" (
  echo Error: psql is not installed or not available in PATH.
  exit /b 1
)
if not "%PG_ISREADY_AVAILABLE%"=="1" (
  echo Error: pg_isready is not installed or not available in PATH.
  exit /b 1
)
exit /b 0

:database_connection_ready
set "PGPASSWORD=%POSTGRES_PASSWORD%"
pg_isready -h "%POSTGRES_HOST%" -p "%POSTGRES_PORT%" -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" >nul 2>nul
exit /b %errorlevel%

:start_postgres
if /I "%DB_RUNNER%"=="local" (
  echo Using local PostgreSQL at %POSTGRES_HOST%:%POSTGRES_PORT%.
  exit /b 0
)

if /I "%DB_RUNNER%"=="wsl" (
  echo Starting PostgreSQL inside WSL...
  if defined WSL_DB_DISTRO (
    wsl.exe -d "%WSL_DB_DISTRO%" sh -lc "sudo -n service postgresql start || service postgresql start"
  ) else (
    wsl.exe sh -lc "sudo -n service postgresql start || service postgresql start"
  )
  if errorlevel 1 (
    echo Error: failed to start PostgreSQL inside WSL.
    echo If your PostgreSQL service lives in a non-default distro, set WSL_DB_DISTRO in .env, for example:
    echo   WSL_DB_DISTRO=Ubuntu-Work
    echo If sudo prompts for a password in WSL, run this once manually:
    echo   wsl.exe -d ^<distro^> sh -lc "sudo service postgresql start"
    exit /b 1
  )
  exit /b 0
)

if /I "%DOCKER_MODE%"=="compose-plugin" (
  docker compose up -d postgres
  exit /b %errorlevel%
)

docker-compose up -d postgres
exit /b %errorlevel%

:wait_for_postgres
echo Waiting for PostgreSQL to become ready...
for /l %%I in (1,1,30) do (
  if /I "%DB_RUNNER%"=="docker" (
    if /I "%DOCKER_MODE%"=="compose-plugin" (
      docker compose exec -T postgres pg_isready -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" >nul 2>nul
    ) else (
      docker-compose exec -T postgres pg_isready -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" >nul 2>nul
    )
  ) else (
    pg_isready -h "%POSTGRES_HOST%" -p "%POSTGRES_PORT%" >nul 2>nul
  )
  if not errorlevel 1 exit /b 0
  timeout /t 2 /nobreak >nul
)

echo Error: PostgreSQL did not become ready in time.
if /I "%DB_RUNNER%"=="wsl" (
  echo The script attempted to start PostgreSQL in WSL but Windows still cannot reach %POSTGRES_HOST%:%POSTGRES_PORT%.
  echo Check the WSL distro status and whether PostgreSQL is configured to listen on localhost.
)
if /I "%DB_RUNNER%"=="local" (
  echo Local PostgreSQL is not reachable at %POSTGRES_HOST%:%POSTGRES_PORT%.
)
exit /b 1

:verify_database_connection
set "PGPASSWORD=%POSTGRES_PASSWORD%"
psql -h "%POSTGRES_HOST%" -p "%POSTGRES_PORT%" -U "%POSTGRES_USER%" -d "%POSTGRES_DB%" -c "SELECT 1;" >nul 2>nul
if not errorlevel 1 exit /b 0

echo Error: PostgreSQL is reachable at %POSTGRES_HOST%:%POSTGRES_PORT%, but database %POSTGRES_DB% with user %POSTGRES_USER% is not ready.
if /I "%DB_RUNNER%"=="wsl" (
  echo If this is the first WSL-based setup, create them once inside WSL:
  if defined WSL_DB_DISTRO (
    echo   wsl.exe -d %WSL_DB_DISTRO% sh -lc "sudo -u postgres createuser -P %POSTGRES_USER%"
    echo   wsl.exe -d %WSL_DB_DISTRO% sh -lc "sudo -u postgres createdb -O %POSTGRES_USER% %POSTGRES_DB%"
  ) else (
    echo   wsl.exe sh -lc "sudo -u postgres createuser -P %POSTGRES_USER%"
    echo   wsl.exe sh -lc "sudo -u postgres createdb -O %POSTGRES_USER% %POSTGRES_DB%"
  )
)
exit /b 1
