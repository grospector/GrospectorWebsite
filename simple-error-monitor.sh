#!/bin/bash

# Simple error-only monitoring script - No dependencies required
# This script captures only console errors and critical issues

echo "🔍 Starting SIMPLE ERROR-ONLY monitoring for Cline..."

# Create log directory
mkdir -p logs

# Initialize error log file
ERROR_LOG="logs/console-errors-only.log"
echo "# Browser Error Log - Started at $(date)" > "$ERROR_LOG"

# Function to get console errors via DevTools API
get_console_errors() {
    local tab_id=$1
    
    # Enable runtime and log domains
    curl -s -X POST "http://localhost:9222/json/runtime/enable" \
        -H "Content-Type: application/json" \
        -d "{\"id\":1,\"method\":\"Runtime.enable\"}" > /dev/null
    
    curl -s -X POST "http://localhost:9222/json/runtime/enable" \
        -H "Content-Type: application/json" \
        -d "{\"id\":2,\"method\":\"Log.enable\"}" > /dev/null
    
    # Get console messages (simplified approach)
    curl -s -X POST "http://localhost:9222/json/runtime/evaluate" \
        -H "Content-Type: application/json" \
        -d '{"id":3,"method":"Runtime.evaluate","params":{"expression":"console.error.toString()"}}' 2>/dev/null
}

# Function to monitor for specific error patterns in the browser
monitor_browser_errors() {
    echo "📊 Starting simple error monitoring..."
    
    while true; do
        timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        
        # Check if app is responding
        if ! curl -s http://localhost:8080 > /dev/null; then
            echo "[$timestamp] ❌ APP ERROR: App is not responding" | tee -a "$ERROR_LOG"
        fi
        
        # Check if Chrome DevTools is responding
        if ! curl -s http://localhost:9222/json > /dev/null; then
            echo "[$timestamp] ❌ CHROME ERROR: DevTools not responding" | tee -a "$ERROR_LOG"
        fi
        
        # Get current tab info and check for console errors in page
        TAB_INFO=$(curl -s http://localhost:9222/json | jq -r '.[] | select(.url | contains("localhost:8080"))')
        
        if [ -z "$TAB_INFO" ]; then
            echo "[$timestamp] ⚠️ WARNING: App tab not found in Chrome" | tee -a "$ERROR_LOG"
        fi
        
        # Simple check for common error indicators
        # Check if page title indicates an error
        PAGE_TITLE=$(echo "$TAB_INFO" | jq -r '.title // empty')
        if [[ "$PAGE_TITLE" == *"Error"* ]] || [[ "$PAGE_TITLE" == *"404"* ]] || [[ "$PAGE_TITLE" == *"500"* ]]; then
            echo "[$timestamp] ❌ PAGE ERROR: Error detected in page title: $PAGE_TITLE" | tee -a "$ERROR_LOG"
        fi
        
        sleep 5
    done
}

# Function to inject error capture into the page
inject_error_capture() {
    echo "📝 Injecting error capture script into browser..."
    
    # Get the tab ID for localhost:8080
    TAB_ID=$(curl -s http://localhost:9222/json | jq -r '.[] | select(.url | contains("localhost:8080")) | .id')
    
    if [ -z "$TAB_ID" ] || [ "$TAB_ID" = "null" ]; then
        echo "❌ Could not find app tab"
        return 1
    fi
    
    echo "✅ Found app tab: $TAB_ID"
    
    # Inject error monitoring script via DevTools
    INJECT_SCRIPT='
    (function() {
        const originalError = console.error;
        const originalWarn = console.warn;
        
        // Store errors in a global array
        window.__browserErrors = window.__browserErrors || [];
        
        console.error = function(...args) {
            originalError.apply(console, args);
            window.__browserErrors.push({
                type: "error",
                message: args.join(" "),
                timestamp: new Date().toISOString()
            });
        };
        
        console.warn = function(...args) {
            originalWarn.apply(console, args);
            if (args.join(" ").toLowerCase().includes("error")) {
                window.__browserErrors.push({
                    type: "warning-error",
                    message: args.join(" "),
                    timestamp: new Date().toISOString()
                });
            }
        };
        
        // Catch unhandled errors
        window.addEventListener("error", function(event) {
            window.__browserErrors.push({
                type: "unhandled",
                message: event.message + " at " + event.filename + ":" + event.lineno,
                timestamp: new Date().toISOString()
            });
        });
        
        console.log("✅ Error monitoring injected");
    })();
    '
    
    # Inject the script
    curl -s -X POST "http://localhost:9222/json/runtime/evaluate" \
        -H "Content-Type: application/json" \
        -d "{\"id\":4,\"method\":\"Runtime.evaluate\",\"params\":{\"expression\":$(echo "$INJECT_SCRIPT" | jq -R .)}}" > /dev/null
    
    echo "✅ Error capture script injected"
}

# Function to check for injected errors
check_injected_errors() {
    echo "🔍 Checking for browser errors..."
    
    while true; do
        timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        
        # Get errors from the injected script
        ERRORS_JSON=$(curl -s -X POST "http://localhost:9222/json/runtime/evaluate" \
            -H "Content-Type: application/json" \
            -d '{"id":5,"method":"Runtime.evaluate","params":{"expression":"JSON.stringify(window.__browserErrors || [])"}}' | \
            jq -r '.result.value // "[]"' 2>/dev/null)
        
        if [ "$ERRORS_JSON" != "[]" ] && [ "$ERRORS_JSON" != "" ] && [ "$ERRORS_JSON" != "null" ]; then
            # Parse and log errors
            echo "$ERRORS_JSON" | jq -r '.[] | "\(.timestamp) ❌ \(.type | ascii_upcase): \(.message)"' | while read -r error_line; do
                if [ ! -z "$error_line" ]; then
                    echo "[$timestamp] $error_line" | tee -a "$ERROR_LOG"
                fi
            done
            
            # Clear the errors array after logging
            curl -s -X POST "http://localhost:9222/json/runtime/evaluate" \
                -H "Content-Type: application/json" \
                -d '{"id":6,"method":"Runtime.evaluate","params":{"expression":"window.__browserErrors = []"}}' > /dev/null
        fi
        
        sleep 3
    done
}

# Cleanup function
cleanup() {
    echo "🧹 Cleaning up error monitoring..."
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Main execution
echo "🎬 Starting SIMPLE ERROR-ONLY browser monitoring..."

# Check if Chrome is running
if ! pgrep -f "remote-debugging-port=9222" > /dev/null; then
    echo "❌ Chrome with remote debugging is not running"
    echo "Please run: ./monitor-browser.sh first"
    exit 1
fi

echo "✅ Chrome with remote debugging detected"

# Wait for Chrome to be ready
sleep 2

# Start monitoring functions
echo "🚀 Starting error monitoring components..."

# Start basic monitoring
monitor_browser_errors &
BASIC_MONITOR_PID=$!

# Try to inject error capture
if inject_error_capture; then
    # Start injected error checking
    check_injected_errors &
    INJECTED_MONITOR_PID=$!
    echo "✅ Advanced error monitoring active"
else
    echo "⚠️ Using basic error monitoring only"
fi

echo "✅ SIMPLE ERROR-ONLY monitoring is active!"
echo "📂 Error logs: $ERROR_LOG"
echo "🎯 MONITORING FOR ERRORS ONLY - No spam!"
echo "⏹️ Press Ctrl+C to stop"
echo ""

# Keep script running
wait
