#!/bin/bash

SERVER_NAME="screech"
BINARY_PATH="./target/debug/$SERVER_NAME"
PID_FILE=".$SERVER_NAME.pid"

start() {
    if [ -f "$PID_FILE" ] && ps -p $(cat "$PID_FILE") >/dev/null 2>&1; then
        echo "Server is already running (PID: $(cat $PID_FILE))"
        return 1
    fi

    echo "Starting $SERVER_NAME server..."
    nohup $BINARY_PATH >server.log 2>&1 &
    echo $! >"$PID_FILE"
    sleep 2

    if ps -p $(cat "$PID_FILE") >/dev/null 2>&1; then
        echo "Server started successfully (PID: $(cat $PID_FILE))"
        echo "Health check: curl http://localhost:3000/health"
    else
        echo "Failed to start server"
        rm -f "$PID_FILE"
        return 1
    fi
}

stop() {
    if [ ! -f "$PID_FILE" ]; then
        echo "Server is not running"
        return 1
    fi

    PID=$(cat "$PID_FILE")
    echo "Stopping server (PID: $PID)..."

    kill $PID 2>/dev/null
    sleep 2

    if ps -p $PID >/dev/null 2>&1; then
        echo "Force killing server..."
        kill -9 $PID 2>/dev/null
    fi

    rm -f "$PID_FILE"
    pkill -f "chromedriver" 2>/dev/null || true
    echo "Server stopped"
}

restart() {
    stop
    sleep 1
    start
}

status() {
    if [ -f "$PID_FILE" ] && ps -p $(cat "$PID_FILE") >/dev/null 2>&1; then
        echo "Server is RUNNING (PID: $(cat $PID_FILE))"
        if curl -s http://localhost:3000/health >/dev/null 2>&1; then
            echo "Server is responding to requests"
        else
            echo "Server is running but not responding"
        fi
    else
        echo "Server is NOT RUNNING"
        rm -f "$PID_FILE"
    fi
}

logs() {
    if [ -f "server.log" ]; then
        tail -f server.log
    else
        echo "No log file found"
    fi
}

case "$1" in
start)
    start
    ;;
stop)
    stop
    ;;
restart)
    restart
    ;;
status)
    status
    ;;
logs)
    logs
    ;;
*)
    echo "Usage: $0 {start|stop|restart|status|logs}"
    exit 1
    ;;
esac
