#!/bin/bash

HOST="127.0.0.1"
PORT=6379

# 尝试连接 Redis 端口
redis-cli -h $HOST -p $PORT ping

if [ $? -eq 0 ]; then
    echo "$HOST:$PORT 🟢 在线"
else
    echo "$HOST:$PORT 🔴 离线或无法访问"
fi