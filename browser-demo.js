// Simple browser automation demo for Bitcoin Wealth Comparison Tool
// This script demonstrates the application features

const puppeteer = require('puppeteer');

async function demoApp() {
    console.log('🚀 Starting Bitcoin Wealth Comparison Tool Demo...\n');
    
    // Launch browser
    const browser = await puppeteer.launch({ 
        headless: false, 
        defaultViewport: { width: 1200, height: 800 },
        args: ['--no-sandbox']
    });
    
    const page = await browser.newPage();
    
    try {
        console.log('📡 Navigating to http://localhost:8080...');
        await page.goto('http://localhost:8080', { waitUntil: 'networkidle2' });
        
        console.log('⏱️ Waiting for WASM application to load...');
        await page.waitForTimeout(3000);
        
        console.log('📸 Taking screenshot of initial state...');
        await page.screenshot({ path: 'demo-1-initial.png', fullPage: true });
        
        console.log('🔍 Looking for Bitcoin input field...');
        const inputSelector = 'input[type="number"]';
        await page.waitForSelector(inputSelector, { timeout: 10000 });
        
        console.log('💰 Entering Bitcoin amount: 1.5 BTC...');
        await page.click(inputSelector);
        await page.type(inputSelector, '1.5');
        
        console.log('🧮 Clicking Calculate Percentile button...');
        const buttonSelector = 'button:contains("Calculate Percentile")';
        await page.click('button[type="button"]');
        
        console.log('⏱️ Waiting for calculation results...');
        await page.waitForTimeout(2000);
        
        console.log('📸 Taking screenshot of results...');
        await page.screenshot({ path: 'demo-2-results.png', fullPage: true });
        
        console.log('📊 Scrolling to view charts...');
        await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
        await page.waitForTimeout(1000);
        
        console.log('📸 Taking screenshot of charts...');
        await page.screenshot({ path: 'demo-3-charts.png', fullPage: true });
        
        console.log('✅ Demo completed successfully!');
        console.log('\n📁 Screenshots saved:');
        console.log('   • demo-1-initial.png - Initial application state');
        console.log('   • demo-2-results.png - After entering 1.5 BTC');
        console.log('   • demo-3-charts.png - Full charts view');
        
    } catch (error) {
        console.log('❌ Demo error:', error.message);
        console.log('📸 Taking error screenshot...');
        await page.screenshot({ path: 'demo-error.png', fullPage: true });
    }
    
    console.log('\n🌐 Application is running at: http://localhost:8080');
    console.log('👆 You can manually interact with it in your browser');
    
    // Keep browser open for manual interaction
    console.log('\n⏸️ Browser will stay open for manual demonstration...');
    // await browser.close();
}

// Run if puppeteer is available, otherwise provide manual instructions
try {
    demoApp();
} catch (error) {
    console.log('🔧 Browser automation not available, but here\'s what you can do manually:\n');
    console.log('1. 🌐 Open http://localhost:8080 in your browser');
    console.log('2. ⏱️ Wait for the app to load (you\'ll see a loading spinner)');
    console.log('3. 💰 Enter a Bitcoin amount (try 1.5 or 0.25)');
    console.log('4. 🧮 Click "Calculate Percentile"');
    console.log('5. 📊 Scroll down to see interactive charts');
    console.log('6. 🎯 Try different amounts to see how rankings change');
}
