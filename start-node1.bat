@echo off
title NovaChain Node 1 (Port 8080)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8080
echo 🌐 API Port: 3001
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8080 --api-port 3001

pause