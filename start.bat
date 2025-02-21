@echo off
setlocal EnableDelayedExpansion

echo Starting cluster setup...

:: Get array of ports from instances
for /f "tokens=* delims=" %%a in ('powershell -NoProfile -Command "$config = Get-Content -Raw 'config.json' | ConvertFrom-Json; $config.instances.port | ForEach-Object { Write-Output $_ }"') do (
    echo Processing port: %%a
    
    :: Update only the top-level port
    powershell -NoProfile -Command "$config = Get-Content -Raw 'config.json' | ConvertFrom-Json; $config.port = [int]'%%a'; $config | ConvertTo-Json -Depth 10 | Set-Content 'config.json'"
    
    :: Start the instance
    echo Starting cargo for port %%a
    start "Cargo Instance %%a" cmd /c "cargo run"
    
    :: Wait a bit before starting next instance
    timeout /t 5 /nobreak > nul
)

echo.
echo Cluster setup complete. Instances are running in separate windows.
echo Use Task Manager to view running processes.
echo.

endlocal