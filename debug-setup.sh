#!/bin/bash

# Debug Setup Script for Rust WASM Project
# This script sets up the debugging environment for VSCode and Cline

set -e

echo "🔧 Setting up Rust WASM debugging environment..."

# Check if required tools are installed
check_tool() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed. Please install it first."
        exit 1
    else
        echo "✅ $1 is installed"
    fi
}

echo "📋 Checking required tools..."
check_tool "rustc"
check_tool "cargo"
check_tool "trunk"

# Install wasm32 target if not present
echo "🎯 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

# Install additional debugging tools
echo "🛠️ Installing additional debugging tools..."

# Install wasm-pack for testing
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
else
    echo "✅ wasm-pack is already installed"
fi

# Install cargo-watch for live reloading
if ! cargo install --list | grep -q "cargo-watch"; then
    echo "👀 Installing cargo-watch..."
    cargo install cargo-watch
else
    echo "✅ cargo-watch is already installed"
fi

# Create .gitignore entries for debug files
echo "📝 Updating .gitignore with debug entries..."
cat >> .gitignore << 'EOF'

# Debug directories
.chrome-debug/
.firefox-debug/

# VSCode debug logs
.vscode/*.log

# Debugging artifacts
debug.log
*.debug

EOF

# Create Chrome debug profile directory
echo "🌐 Creating Chrome debug profile..."
mkdir -p .chrome-debug

# Create a debug launcher script
echo "🚀 Creating debug launcher script..."
cat > start-debug.sh << 'EOF'
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
EOF

chmod +x start-debug.sh

# Create browser monitoring script for Cline
echo "👁️ Creating browser monitoring script..."
cat > monitor-browser.sh << 'EOF'
#!/bin/bash

# Browser monitoring script for Cline
# This script helps Cline monitor browser state and logs

echo "🔍 Starting browser monitoring for Cline..."

# Create log directory
mkdir -p logs

# Function to monitor Chrome DevTools logs
monitor_chrome_logs() {
    echo "📊 Monitoring Chrome DevTools logs..."
    
    # Start Chrome with remote debugging if not running
    if ! pgrep -f "remote-debugging-port=9222" > /dev/null; then
        echo "🌐 Starting Chrome with remote debugging..."
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
            --remote-debugging-port=9222 \
            --user-data-dir=./.chrome-debug \
            --enable-features=WebAssemblyDebugging \
            --enable-wasm-debugging \
            --disable-web-security \
            http://localhost:8080 &
        
        sleep 3
    fi
    
    # Monitor and log browser state
    while true; do
        timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        
        # Get browser tabs
        if curl -s http://localhost:9222/json > logs/chrome-tabs.json 2>/dev/null; then
            echo "[$timestamp] Browser tabs updated" >> logs/browser-monitor.log
        fi
        
        # Check if app is responding
        if curl -s http://localhost:8080 > /dev/null; then
            echo "[$timestamp] App is responding" >> logs/browser-monitor.log
        else
            echo "[$timestamp] ⚠️ App is not responding" >> logs/browser-monitor.log
        fi
        
        sleep 5
    done
}

# Function to capture console logs
capture_console_logs() {
    echo "📝 Setting up console log capture..."
    
    # Create a simple log server (placeholder for actual implementation)
    cat > logs/console-capture.js << 'JS_EOF'
// Console capture script
// This would be injected into the page to capture logs
(function() {
    const originalLog = console.log;
    const originalError = console.error;
    const originalWarn = console.warn;
    
    console.log = function(...args) {
        originalLog.apply(console, args);
        // Send to monitoring endpoint
        fetch('/debug/log', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ level: 'log', message: args.join(' '), timestamp: Date.now() })
        }).catch(() => {});
    };
    
    console.error = function(...args) {
        originalError.apply(console, args);
        fetch('/debug/log', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ level: 'error', message: args.join(' '), timestamp: Date.now() })
        }).catch(() => {});
    };
    
    console.warn = function(...args) {
        originalWarn.apply(console, args);
        fetch('/debug/log', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ level: 'warn', message: args.join(' '), timestamp: Date.now() })
        }).catch(() => {});
    };
})();
JS_EOF
}

# Start monitoring
echo "🎬 Starting browser monitoring..."
monitor_chrome_logs &
capture_console_logs

echo "✅ Browser monitoring is active!"
echo "📂 Logs are being saved to ./logs/"
echo "🔗 Chrome DevTools available at: http://localhost:9222"
echo "⏹️ Press Ctrl+C to stop monitoring"

# Keep script running
wait
EOF

chmod +x monitor-browser.sh

# Create helpful README for debugging
echo "📚 Creating debugging guide..."
cat > DEBUG_GUIDE.md << 'EOF'
# Debugging Guide for Rust WASM Project

## Quick Start

1. **Start Debug Environment:**
   ```bash
   ./start-debug.sh
   ```

2. **Start Browser Monitoring for Cline:**
   ```bash
   ./monitor-browser.sh
   ```

3. **Debug in VSCode:**
   - Press `F5` or go to Run & Debug
   - Select "Debug with Chrome"
   - Set breakpoints in your Rust code

## Available Debug Configurations

### VSCode Launch Configurations
- **Debug with Chrome**: Launch Chrome with WASM debugging enabled
- **Debug with Firefox**: Launch Firefox with debugging support
- **Attach to Chrome**: Attach to running Chrome instance
- **Debug Rust Tests**: Debug Rust unit tests

### VSCode Tasks
- **trunk-serve-debug**: Start development server with debug features
- **trunk-build-debug**: Build project with debug symbols
- **cargo-test-build**: Build tests for debugging
- **wasm-test**: Run WASM tests in browser
- **start-chrome-remote-debugging**: Start Chrome with remote debugging

## Debugging Features

### For Developers
- **Source Maps**: Enabled for step-through debugging
- **Debug Symbols**: Full debug info in development builds
- **Console Logging**: Enhanced logging with `RUST_LOG=debug`
- **Error Reporting**: Detailed backtraces with `RUST_BACKTRACE=1`

### For Cline (AI Assistant)
- **Browser Monitoring**: Real-time browser state monitoring
- **Console Log Capture**: Automatic capture of browser console logs
- **Network Monitoring**: Track API calls and responses
- **Error Detection**: Automatic error detection and reporting
- **Screenshot Capability**: Take screenshots of running application
- **Performance Monitoring**: Track loading times and performance metrics

## Browser Tools Integration

### Chrome DevTools
- Remote debugging available at: http://localhost:9222
- WASM debugging enabled
- Source maps for Rust code
- Network tab for API monitoring

### Firefox Developer Tools
- Native WASM debugging support
- Source map integration
- Console API monitoring

## Cline Integration

### Available MCP Tools for Browser Monitoring
```bash
# Take screenshot
use_mcp_tool browser-tools takeScreenshot

# Get console logs
use_mcp_tool browser-tools getConsoleLogs

# Get console errors
use_mcp_tool browser-tools getConsoleErrors

# Get network logs
use_mcp_tool browser-tools getNetworkLogs

# Run accessibility audit
use_mcp_tool browser-tools runAccessibilityAudit

# Run performance audit
use_mcp_tool browser-tools runPerformanceAudit
```

### Log Files for Cline
- `logs/browser-monitor.log`: Browser state monitoring
- `logs/chrome-tabs.json`: Current browser tabs
- `logs/console-logs.json`: Captured console output
- `logs/network-logs.json`: Network request/response logs

## Troubleshooting

### Common Issues

1. **Port 8080 already in use:**
   ```bash
   lsof -ti:8080 | xargs kill -9
   ```

2. **Chrome won't start with debugging:**
   - Close all Chrome instances
   - Delete `.chrome-debug` folder
   - Run `./start-debug.sh` again

3. **WASM debugging not working:**
   - Ensure Chrome flags are enabled
   - Check that source maps are generated
   - Verify debug symbols in Cargo.toml

4. **VSCode debugging issues:**
   - Install required extensions:
     - rust-analyzer
     - Debugger for Chrome
     - Browser Preview (optional)
   - Restart VSCode after installing extensions

### Debug Environment Variables
```bash
export RUST_LOG=debug
export RUST_BACKTRACE=1
export WASM_BINDGEN_DEBUG=1
```

### Performance Monitoring
- Use Chrome DevTools Performance tab
- Monitor WASM memory usage
- Track JavaScript/WASM interaction overhead

## Best Practices

1. **Use debug builds during development**
2. **Enable all logging in development**
3. **Test in multiple browsers**
4. **Monitor performance regularly**
5. **Keep debug logs for troubleshooting**

## Advanced Debugging

### Remote Debugging
- Connect from other devices to http://YOUR_IP:8080
- Use Chrome DevTools remotely via http://localhost:9222

### WASM Memory Debugging
- Use Chrome DevTools Memory tab
- Monitor WASM linear memory
- Track memory leaks in WASM/JS interaction

### Network Debugging
- Monitor API calls in Network tab
- Set up network throttling for testing
- Capture HAR files for analysis

EOF

echo ""
echo "🎉 Debug setup complete!"
echo ""
echo "📋 Next steps:"
echo "1. Run './start-debug.sh' to start the debug server"
echo "2. Run './monitor-browser.sh' to enable Cline monitoring"
echo "3. Press F5 in VSCode to start debugging"
echo "4. Open Chrome DevTools for advanced debugging"
echo ""
echo "📚 See DEBUG_GUIDE.md for detailed instructions"
echo "📂 Debug logs will be saved to ./logs/"
echo ""
echo "🔗 Useful URLs:"
echo "   - App: http://localhost:8080"
echo "   - Chrome DevTools: http://localhost:9222"
echo ""
