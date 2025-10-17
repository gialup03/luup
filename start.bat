@echo off
echo Starting Luup...
cd /d "%~dp0"
call npm run tauri:dev
