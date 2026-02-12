use death_note::identification::WeChatShinigamiEye;
use death_note::identification::traits::ShinigamiEye;

#[tokio::test]
async fn test_wechat_identifier_extracts_fields() {
    let eye = WeChatShinigamiEye::new();
    let results = eye.identify().await;

    let mut pairs: Vec<(String, String)> = results
        .iter()
        .map(|r| (r.source().to_string(), r.name().to_string()))
        .collect();
    pairs.sort();

    assert_eq!(
        pairs,
        vec![
            ("微信-wxid".to_string(), "wxid_test_123456".to_string()),
            ("微信-微信号".to_string(), "test_wechat_id".to_string()),
            ("微信-手机号".to_string(), "13800000000".to_string()),
            ("微信-昵称".to_string(), "TestNick".to_string()),
        ]
    );

    for result in results {
        if result.source() == "微信-wxid" || result.source() == "微信-昵称" {
            assert!(!result.is_blacklisted());
        }
    }
}
