use tauri_app_lib::api::admin::resolve_network_url;

#[tokio::test]
async fn test_resolve_network_url() {
    let url = resolve_network_url(3000).unwrap();
    println!("Got URL: {}", url);
    assert!(url.starts_with("http://"));
    assert!(url.contains(":3000"));
    
    // 验证是否包含合法的 IP 格式
    let ip_part = url.trim_start_matches("http://").split(':').next().unwrap();
    let parts: Vec<&str> = ip_part.split('.').collect();
    assert_eq!(parts.len(), 4, "IP should have 4 parts: {}", ip_part);
}
