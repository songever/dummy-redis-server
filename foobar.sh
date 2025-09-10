#!/bin/bash

HOST="127.0.0.1"
PORT=6379

# 测试 SET 命令
echo "SET foo bar"
redis-cli -h $HOST -p $PORT set foo bar

# 测试 GET 命令
echo "GET foo"
redis-cli -h $HOST -p $PORT get foo

# 检查结果
VALUE=$(redis-cli -h $HOST -p $PORT get foo)
if [ "$VALUE" = "bar" ]; then
    echo "GET/SET 测试 🟢 成功"
else
    echo "GET/SET 测试 🔴 失败"
fi