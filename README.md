# cloudflare worker: binance api proxy
首先安装一下 Rust Wasm 的 Toolchain，用于编译 Wasm  
rustup target add wasm32-unknown-unknown  

顺便安装下 cargo-generate:  
cargo install cargo-generate  


随后根据 workers-rs 的模板创建项目
cargo generate cloudflare/workers-rs

## 本地测试
npx wrangler dev  
访问 http://localhost:8787/api/v3/ticker/price?symbols=[%22BTCUSDT%22,%22ETHUSDT%22,%22SOLUSDT%22]

## 部署到worker
npx wrangler deploy  
Wrangler会让你登录 Cloudflare 账户，跳转浏览器登录即可

## Error 😂
"Service unavailable from a restricted location according to 'b. Eligibility' in https://www.binance.com/en/terms. Please contact customer service if you believe you received this message in error."