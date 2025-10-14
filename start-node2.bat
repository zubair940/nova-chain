@echo off
title NovaChain Node 2 (Port 8081)
echo 🚀 Starting NovaChain Node 2...
echo 🌐 P2P Port: 8081
echo 🌐 API Port: 3002
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8081 --api-port 3002

pause