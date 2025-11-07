@echo off
title NovaChain Node 1 (Port 8084)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8084
echo 🌐 API Port: 3005
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8084 --api-port 3005

pause
