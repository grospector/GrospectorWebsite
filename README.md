# Bitcoin Wealth Comparison Tool

A comprehensive web application that visualizes global Bitcoin wealth distribution and allows users to compare their holdings against the world.

## 🎯 Project Purpose

This tool provides insights into Bitcoin wealth inequality by:
- Visualizing the global distribution of Bitcoin holdings
- Allowing users to see where their Bitcoin wealth ranks globally
- Providing educational insights about cryptocurrency wealth concentration
- Maintaining complete user privacy through client-side calculations

## 🌟 Core Features

### 📊 Global Distribution Visualization
- Interactive charts showing Bitcoin distribution across all addresses
- Logarithmic scaling to handle extreme wealth concentration
- Real-time percentile breakdowns and statistics

### 🔍 Personal Wealth Comparison
- Private input for Bitcoin amount (never transmitted to servers)
- Instant percentile calculation and ranking
- Visual positioning on global distribution chart
- Comparison statistics ("You're wealthier than X% of Bitcoin holders")

### 📈 Statistical Dashboard
- Key metrics: median holdings, wealth concentration ratios
- Percentile thresholds (90th, 95th, 99th percentiles)
- Total address count and distribution analytics
- Historical trend data (when available)

### 🎓 Educational Content
- Explanations of Bitcoin wealth distribution patterns
- Information about data sources and methodology
- Privacy policy and security explanations
- Context about cryptocurrency wealth inequality

## 🏗️ Technical Architecture

### Frontend Stack
- **Framework:** Yew or Leptos (Rust web framework compiled to WASM)
- **Build Tool:** Trunk for WASM compilation and asset bundling
- **Styling:** Tailwind CSS with Stylist for CSS-in-Rust
- **Charts:** Plotters.rs for data visualization with web-sys for canvas/SVG rendering
- **State Management:** Rust structs with Rc/RefCell for shared state

### Why Rust + WASM?
- **Performance:** Near-native performance for complex calculations
- **Safety:** Memory safety without garbage collection
- **Reliability:** Compile-time error checking prevents runtime bugs
- **Efficiency:** Smaller bundle sizes and faster execution than JavaScript
- **Security:** Rust's safety guarantees enhance application security

### Data Sources
- **Primary:** BitInfoCharts API for Bitcoin distribution data
- **Secondary:** Bitcoin Rich List APIs for additional context
- **Backup:** On-chain analysis services for data redundancy

### Privacy & Security
- **Client-Side Only:** All calculations performed in the browser
- **No Data Transmission:** User inputs never leave their device
- **Data Validation:** Comprehensive input sanitization and validation
- **Security Headers:** HTTPS enforcement and CSP implementation

## 🚀 Getting Started

### Prerequisites
- Rust toolchain (rustup, cargo)
- Trunk build tool (`cargo install trunk`)
- wasm-pack for WebAssembly compilation
- Modern web browser with WebAssembly support

### Installation
```bash
# Clone the repository
git clone [repository-url]

# Navigate to project directory
cd bitcoin-wealth-comparison

# Install Rust dependencies
cargo build

# Install Trunk (if not already installed)
cargo install trunk

# Start development server
trunk serve
```

### Development Commands
```bash
trunk serve          # Start development server with live reload
trunk build          # Build for production (WASM + assets)
cargo test           # Run Rust unit tests
cargo clippy         # Run Rust linter
cargo fmt            # Format Rust code
```

## 📁 Project Structure

```
src/
├── components/           # Yew/Leptos components
│   ├── charts/          # Chart components (.rs files)
│   ├── forms/           # Form components
│   ├── ui/              # UI components
│   └── layout/          # Layout components
├── hooks/               # Custom Rust hooks/utilities
├── services/            # API and data services
├── types/               # Rust type definitions and structs
├── utils/               # Utility functions
└── styles/              # SCSS/CSS styles
```

## 🎨 Design System

### Color Palette
- **Primary:** Bitcoin Orange (#F7931A)
- **Secondary:** Dark Blue (#1E3A8A)
- **Accent:** Green (#10B981) for positive values
- **Neutral:** Gray scale for backgrounds and text

### Typography
- **Headers:** Inter Bold for titles and headings
- **Body:** Inter Regular for general text
- **Monospace:** JetBrains Mono for numerical data

### Responsive Design
- **Mobile First:** Optimized for mobile devices
- **Breakpoints:** 320px (mobile), 768px (tablet), 1024px+ (desktop)
- **Accessibility:** WCAG 2.1 AA compliant

## 📊 Data Methodology

### Distribution Calculation
- Uses cumulative distribution functions for accurate percentile calculations
- Accounts for address consolidation and exchange wallets
- Handles edge cases like empty addresses and dust amounts

### Data Sources
- **Real-time Data:** Updated daily from multiple reliable sources
- **Historical Data:** Trend analysis when available
- **Backup Systems:** Local fallback data for offline functionality

### Privacy Measures
- All calculations performed client-side
- No user data stored or transmitted
- Clear privacy notices and explanations
- Optional local storage for user preferences only

## 🧪 Testing Strategy

### Test Coverage
- **Unit Tests:** Component and utility function testing with cargo test
- **Integration Tests:** API and data flow testing with wasm-bindgen-test
- **End-to-End Tests:** Complete user journey testing with browser automation
- **Performance Tests:** Load time and WASM execution benchmarks

### Quality Assurance
- **Code Quality:** Rustfmt and Clippy for consistent code style
- **Type Safety:** Rust's compile-time type checking and borrow checker
- **Cross-Browser:** WASM compatibility testing across major browsers
- **Accessibility:** Screen reader and keyboard navigation testing

## 📈 Performance Targets

### Loading Performance
- **Initial Load:** < 2 seconds on 3G connection (WASM advantage)
- **Chart Rendering:** < 500ms for initial display (Rust performance)
- **Data Updates:** < 200ms for real-time updates (efficient calculations)

### Runtime Performance
- **Animations:** 60fps smooth chart interactions
- **Memory Usage:** < 50MB for optimal performance (no GC overhead)
- **Bundle Size:** < 1MB compressed WASM + assets (smaller than JS equivalent)
- **Calculations:** Near-native speed for percentile computations

## 🔒 Security & Privacy

### Privacy Commitments
- **No Data Collection:** User inputs never leave their device
- **Client-Side Processing:** All calculations performed locally
- **Transparent Sources:** Clear information about data sources
- **User Control:** Users control all their data and preferences

### Security Measures
- **Input Validation:** Comprehensive sanitization of all inputs
- **XSS Prevention:** Protection against cross-site scripting
- **HTTPS Only:** Secure communication protocols
- **Security Headers:** CSP and other security headers implemented

## 🌐 Deployment

### Production Build
- **Optimized Assets:** Minified and compressed for fast loading
- **CDN Distribution:** Global content delivery for optimal performance
- **Static Hosting:** Deployed on Netlify/Vercel for reliability
- **CI/CD Pipeline:** Automated testing and deployment

### Environment Support
- **Modern Browsers:** Chrome, Firefox, Safari, Edge (latest versions)
- **Mobile Devices:** iOS Safari, Android Chrome
- **Accessibility:** Screen readers and keyboard navigation support

## 📚 Documentation

### User Documentation
- **Getting Started:** How to use the tool effectively
- **Feature Explanations:** Detailed descriptions of all features
- **FAQ:** Common questions and answers
- **Privacy Policy:** Comprehensive privacy information

### Developer Documentation
- **API Reference:** Complete API documentation
- **Component Library:** Reusable component documentation
- **Contributing Guide:** How to contribute to the project
- **Architecture Guide:** Technical architecture explanation

## 🔮 Future Roadmap

### Planned Features
- **Historical Analysis:** Bitcoin wealth distribution over time
- **Multiple Currencies:** Support for other cryptocurrencies
- **Advanced Analytics:** More sophisticated statistical analysis
- **Mobile App:** Native mobile applications
- **API Access:** Developer API for data access

### Scalability Considerations
- **Microservices:** Scalable backend architecture
- **Database Optimization:** Efficient data storage and retrieval
- **Caching Strategies:** Improved performance through caching
- **Load Balancing:** High availability and performance

## 📊 Success Metrics

### User Engagement
- **Time on Site:** Average session duration
- **Interaction Rate:** Chart and feature usage
- **Return Visits:** User retention and repeat usage
- **Sharing:** Social media and referral metrics

### Technical Performance
- **Page Load Speed:** Core Web Vitals compliance
- **API Response Times:** Sub-second data loading
- **Error Rates:** Minimal error occurrence
- **Uptime:** 99.9% availability target

## 🤝 Contributing

We welcome contributions from the community! Please see our [Contributing Guide](CONTRIBUTING.md) for details on how to get started.

### Ways to Contribute
- **Bug Reports:** Help us identify and fix issues
- **Feature Requests:** Suggest new features and improvements
- **Code Contributions:** Submit pull requests for bug fixes and features
- **Documentation:** Improve documentation and user guides

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Data Providers:** BitInfoCharts and other data sources
- **Community:** Open source contributors and Bitcoin community
- **Libraries:** Rust, Yew/Leptos, Plotters.rs, and other dependencies
- **Inspiration:** Bitcoin wealth inequality research and analysis

---

**Built with ❤️ for the Bitcoin community**

*This tool is designed to provide educational insights into Bitcoin wealth distribution while maintaining complete user privacy and data security.*