@echo off
title NovaChain Node 1 (Port 8086)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8086
echo 🌐 API Port: 3007
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8086 --api-port 3007

pause
