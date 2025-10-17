@echo off
echo Starting Adventure Game...
cd /d "%~dp0"
call npm run tauri:dev
