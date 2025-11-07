@echo off
title NovaChain Node 1 (Port 8087)
echo 🚀 Starting NovaChain Node 1...
echo 🌐 P2P Port: 8087
echo 🌐 API Port: 3008
echo.

cd /d "C:\Users\MR.R\Desktop\NovaChain\novachain-node"
cargo run -- --port 8087 --api-port 3008

pause
