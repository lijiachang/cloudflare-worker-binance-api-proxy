use worker::*;
use reqwest::Client;

async fn proxy_binance(path: &str, query: &str) -> Result<Response> {
    let client = Client::new();
    let binance_url = format!("https://api.binance.com{}", path);
    let url = if !query.is_empty() {
        format!("{}?{}", binance_url, query)
    } else {
        binance_url
    };
    console_log!("binance url:{:?}", &url);

    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.bytes().await.map_err(|e| Error::RustError(e.to_string()))?;

            // 将响应体转换为字符串并打印到日志
            let body_str = String::from_utf8_lossy(&body);
            console_log!("Response body: {:?}", body_str);

            let mut response = Response::from_bytes(Vec::from(body))?
                .with_status(u16::from(status));

            // 设置 Content-Type 为 application/json
            response.headers_mut().set("Content-Type", "application/json")?;

            Ok(response)
        }
        Err(e) => Response::error(e.to_string(), 500)
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/", |_, _| async move {
            Response::ok("Crypto Price Proxy. Use /api/... to access Binance API.")
        })
        .get_async("/api/*path", |req, ctx| async move {
            let binding = "".to_string();
            let path = ctx.param("path").unwrap_or(&binding);
            let url = req.url()?;
            let query = url.query().unwrap_or("");
            console_log!("path:{:?}, url:{:?}, query:{:?}", path, url.to_string(), query);
            proxy_binance(format!("/api/{}", path).as_str(), query).await
        })
        .run(req, env)
        .await
}