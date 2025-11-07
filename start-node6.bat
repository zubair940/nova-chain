@echo off
title NovaChain Node 1 (Port 8085)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8085
echo 🌐 API Port: 3006
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8085 --api-port 3006

pause
