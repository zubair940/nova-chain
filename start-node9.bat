@echo off
title NovaChain Node 1 (Port 8088)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8088
echo 🌐 API Port: 3009
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8088 --api-port 3009

pause
