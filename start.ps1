# Luup Startup Script
Write-Host "Starting Luup..." -ForegroundColor Cyan

# Check if node_modules exists
if (-not (Test-Path "node_modules")) {
    Write-Host "Installing dependencies..." -ForegroundColor Yellow
    npm install
}

# Start Tauri development server
Write-Host "Launching Tauri development server..." -ForegroundColor Green
npm run tauri:dev
