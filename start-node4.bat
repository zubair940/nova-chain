@echo off
title NovaChain Node 1 (Port 8083)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8083
echo 🌐 API Port: 3004
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8083 --api-port 3004

pause
