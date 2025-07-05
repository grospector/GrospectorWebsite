# Cline Debugging Guide for Rust WASM Project

## 🎯 Overview

This guide shows you how to set up a complete debugging environment where **Cline can read and debug from your real browser** while you develop your Rust WebAssembly application.

## 🚀 Quick Setup (3 Steps)

### Step 1: Start the Debug Environment
```bash
# Terminal 1: Start the debug server
./start-debug.sh
```

This will:
- ✅ Start trunk serve with debug symbols
- ✅ Enable RUST_LOG=debug for detailed logging
- ✅ Set up source maps for browser debugging
- ✅ Open http://localhost:8080

### Step 2: Enable Browser Monitoring for Cline
```bash
# Terminal 2: Start browser monitoring
./monitor-browser.sh
```

This will:
- ✅ Start Chrome with remote debugging on port 9222
- ✅ Enable WASM debugging features
- ✅ Create log files that Cline can read
- ✅ Monitor browser state in real-time

### Step 3: Start VSCode Debugging
- Press `F5` in VSCode
- Select "Debug with Chrome"
- Set breakpoints in your Rust code
- Debug step-by-step through your WASM code

## 🔧 How Cline Can Monitor Your App

### Real-Time Browser Monitoring

Once the setup is running, Cline can use these MCP tools to monitor your application:

```bash
# Take screenshots of the running app
use_mcp_tool browser-tools takeScreenshot

# Get console logs (errors, warnings, info)
use_mcp_tool browser-tools getConsoleLogs
use_mcp_tool browser-tools getConsoleErrors

# Monitor network requests (API calls, resource loading)
use_mcp_tool browser-tools getNetworkLogs
use_mcp_tool browser-tools getNetworkErrors

# Run performance audits
use_mcp_tool browser-tools runPerformanceAudit

# Run accessibility audits
use_mcp_tool browser-tools runAccessibilityAudit

# Run SEO audits
use_mcp_tool browser-tools runSEOAudit

# Debug mode for detailed analysis
use_mcp_tool browser-tools runDebuggerMode
```

### Log File Monitoring

Cline can also read these automatically generated log files:

```bash
# Browser state monitoring
cat logs/browser-monitor.log

# Chrome DevTools tabs information
cat logs/chrome-tabs.json

# Console output capture
cat logs/console-logs.json

# Network request/response logs
cat logs/network-logs.json
```

## 📊 Debugging Workflow with Cline

### 1. Development Debugging
```bash
# You: Start coding in Rust
# Cline: Monitor the browser for errors

use_mcp_tool browser-tools getConsoleErrors
use_mcp_tool browser-tools takeScreenshot
```

### 2. Performance Analysis
```bash
# You: Notice slow loading
# Cline: Analyze performance

use_mcp_tool browser-tools runPerformanceAudit
use_mcp_tool browser-tools getNetworkLogs
```

### 3. Bug Investigation
```bash
# You: Report a bug
# Cline: Investigate the issue

use_mcp_tool browser-tools takeScreenshot
use_mcp_tool browser-tools getConsoleLogs
use_mcp_tool browser-tools runDebuggerMode
```

### 4. Code Review
```bash
# You: Want feedback on UI/UX
# Cline: Analyze accessibility and SEO

use_mcp_tool browser-tools runAccessibilityAudit
use_mcp_tool browser-tools runSEOAudit
use_mcp_tool browser-tools takeScreenshot
```

## 🛠️ VSCode Integration

### Debug Configurations Available

1. **Debug with Chrome** (Recommended)
   - Launches Chrome with WASM debugging
   - Source maps enabled for Rust code
   - Breakpoints work in Rust source files

2. **Debug with Firefox**
   - Alternative browser debugging
   - Good WASM support

3. **Attach to Chrome**
   - Attach to running Chrome instance
   - Use when Chrome is already open

4. **Debug Rust Tests**
   - Debug unit tests
   - LLDB integration for native debugging

### Debugging Features

- **✅ Step-through debugging**: Set breakpoints in Rust code
- **✅ Variable inspection**: Inspect Rust variables in browser
- **✅ Call stack**: See Rust function calls
- **✅ Console integration**: console.log from Rust appears in browser
- **✅ Hot reload**: Changes rebuild and reload automatically
- **✅ Error reporting**: Detailed Rust error messages

## 🌐 Browser Tools Setup

### Chrome DevTools Integration
- **URL**: http://localhost:9222
- **Features**: WASM debugging, source maps, performance profiling
- **Flags**: WebAssemblyDebugging, WASM debugging enabled

### Source Maps
- Generated automatically in debug builds
- Maps WASM back to original Rust source
- Enables breakpoints in Rust files

### Console Logging
```rust
// In your Rust code
use log::{info, warn, error};
use wasm_logger;

// Initialize logger (in main or lib.rs)
wasm_logger::init(wasm_logger::Config::default());

// Use logging throughout your code
info!("Bitcoin data loaded successfully");
warn!("API rate limit approaching");
error!("Failed to fetch distribution data: {}", err);
```

## 📁 Project Structure for Debugging

```
GrospectorWebsite/
├── .vscode/                    # VSCode debug configuration
│   ├── launch.json            # Debug launch configs
│   ├── tasks.json             # Build and serve tasks
│   └── settings.json          # Rust/WASM settings
├── logs/                      # Auto-generated debug logs
│   ├── browser-monitor.log    # Browser state monitoring
│   ├── chrome-tabs.json       # Current browser tabs
│   ├── console-logs.json      # Console output
│   └── network-logs.json      # Network requests
├── .chrome-debug/             # Chrome debug profile
├── src/                       # Your Rust source code
├── start-debug.sh            # Start debug environment
├── monitor-browser.sh        # Enable Cline monitoring
└── DEBUG_GUIDE.md           # Detailed debugging instructions
```

## 🔍 Advanced Debugging Techniques

### Memory Debugging
```bash
# Use Chrome DevTools Memory tab
# Monitor WASM linear memory usage
# Track memory leaks in WASM/JS boundary
```

### Performance Profiling
```bash
# Use Chrome DevTools Performance tab
# Profile WASM execution
# Analyze JavaScript/WASM interaction overhead
```

### Network Analysis
```bash
# Monitor API calls to Bitcoin services
# Check response times and errors
# Analyze data transfer sizes
```

## 🚨 Troubleshooting

### Common Issues and Solutions

1. **Port 8080 already in use**
   ```bash
   lsof -ti:8080 | xargs kill -9
   ./start-debug.sh
   ```

2. **Chrome won't start with debugging**
   ```bash
   # Close all Chrome instances
   pkill -f "Google Chrome"
   # Delete debug profile
   rm -rf .chrome-debug
   # Restart
   ./monitor-browser.sh
   ```

3. **WASM debugging not working**
   ```bash
   # Check Chrome flags are enabled
   chrome://flags/#enable-webassembly-debugging
   # Verify source maps are generated
   ls dist/*.map
   ```

4. **VSCode debugging issues**
   ```bash
   # Install required extensions
   # rust-analyzer
   # Debugger for Chrome
   # Restart VSCode
   ```

5. **Cline can't connect to browser**
   ```bash
   # Ensure Chrome is running with remote debugging
   curl http://localhost:9222/json
   # Should return browser tabs information
   ```

## 📈 Monitoring Workflow

### Continuous Monitoring
1. **Start both debug scripts** (start-debug.sh & monitor-browser.sh)
2. **Develop normally** in VSCode
3. **Cline automatically monitors**:
   - Console errors and warnings
   - Network failures
   - Performance issues
   - Accessibility problems
   - Browser crashes

### On-Demand Analysis
```bash
# When you need Cline's help:
"Take a screenshot of the current app state"
"Check for any console errors"
"Run a performance audit"
"Analyze the network requests"
```

## 🎯 Benefits of This Setup

### For You (Developer)
- **Comprehensive debugging**: Step through Rust code in browser
- **Real-time feedback**: See changes immediately
- **Professional tools**: Full Chrome DevTools integration
- **Error reporting**: Detailed Rust backtraces

### For Cline (AI Assistant)
- **Real-time monitoring**: See what you see in the browser
- **Automatic error detection**: Catch issues as they happen
- **Performance insights**: Analyze app performance
- **Visual feedback**: Screenshots of app state
- **Network analysis**: Monitor API calls and responses

### For Collaboration
- **Shared context**: Both you and Cline see the same state
- **Immediate feedback**: Issues caught and reported instantly
- **Documentation**: All debugging sessions logged
- **Reproducible**: Debug setups are consistent

## 🔄 Daily Workflow

```bash
# Morning setup (once per day)
./start-debug.sh          # Terminal 1
./monitor-browser.sh      # Terminal 2

# Development (throughout the day)
# - Code in VSCode
# - Set breakpoints with F5
# - Ask Cline to monitor browser
# - Cline provides real-time feedback

# Evening cleanup (optional)
pkill -f trunk
pkill -f "Google Chrome.*remote-debugging"
```

## 📚 Additional Resources

- **Chrome DevTools**: https://developers.google.com/web/tools/chrome-devtools
- **WASM Debugging**: https://developer.chrome.com/blog/wasm-debugging-2020/
- **Rust WASM Book**: https://rustwasm.github.io/docs/book/
- **VSCode Debugging**: https://code.visualstudio.com/docs/editor/debugging

---

## ✅ Quick Verification

To verify everything is working:

1. **Check debug server**: http://localhost:8080 (should show your app)
2. **Check Chrome DevTools**: http://localhost:9222 (should show remote debugging)
3. **Check VSCode**: F5 should launch debugger
4. **Check Cline**: Use `use_mcp_tool browser-tools takeScreenshot`

🎉 **You now have a complete debugging environment where Cline can monitor and debug your Rust WASM application in real-time!**
