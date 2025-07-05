#!/bin/bash

# Debug launcher script
echo "🔥 Starting debug environment..."

# Kill any existing processes on port 8080
echo "🧹 Cleaning up existing processes..."
lsof -ti:8080 | xargs kill -9 2>/dev/null || true

# Start trunk serve in debug mode
echo "🏗️ Starting trunk serve in debug mode..."
RUST_LOG=error RUST_BACKTRACE=1 trunk serve --port 8080 --address 127.0.0.1 &
TRUNK_PID=$!

# Wait for server to start
echo "⏳ Waiting for server to start..."
sleep 5

# Check if server is running
if curl -s http://localhost:8080 > /dev/null; then
    echo "✅ Server is running at http://localhost:8080"
    echo "🎯 Trunk PID: $TRUNK_PID"
    echo "📝 Server logs are available in the terminal"
    echo ""
    echo "🔧 Debug setup complete! You can now:"
    echo "   1. Open http://localhost:8080 in Chrome with debugging enabled"
    echo "   2. Use F5 in VSCode to start debugging"
    echo "   3. Use Cline browser tools to monitor the application"
    echo ""
    echo "⏹️ Press Ctrl+C to stop the debug server"
    
    # Keep script running
    wait $TRUNK_PID
else
    echo "❌ Failed to start server"
    kill $TRUNK_PID 2>/dev/null || true
    exit 1
fi
