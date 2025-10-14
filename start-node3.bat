@echo off
title NovaChain Node 3 (Port 8082)
echo 🚀 Starting NovaChain Node 3...
echo 🌐 P2P Port: 8082
echo 🌐 API Port: 3003
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8082 --api-port 3003

pause