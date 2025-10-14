@echo off
title NovaChain Network Starter
echo 🌐 Starting NovaChain Multi-Node Network...
echo.

echo 🚀 Starting Node 1 (Port 8080, 3001)...
start "Node1" start-node1.bat

timeout /t 5

echo 🚀 Starting Node 2 (Port 8081, 3002)...
start "Node2" start-node2.bat

timeout /t 5

echo 🚀 Starting Node 3 (Port 8082, 3003)...
start "Node3" start-node3.bat

echo.
echo ✅ All nodes started successfully!
echo 📍 Node 1: http://localhost:3001/explorer
echo 📍 Node 2: http://localhost:3002/explorer  
echo 📍 Node 3: http://localhost:3003/explorer
echo.
pause