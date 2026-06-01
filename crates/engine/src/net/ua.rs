use crate::BrowserProfile;
use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a pseudo-random realistic browser profile from thousands of combinations.
pub fn generate_random_profile() -> BrowserProfile {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    let selected_browser = match nanos % 4 {
        0 => "chrome",
        1 => "edge",
        2 => "firefox",
        _ => "safari",
    };

    let selected_os = match (nanos >> 2) % 5 {
        0 => "windows",
        1 => "mac",
        2 => "linux",
        3 => "ios",
        _ => "android",
    };

    let chrome_major = 120 + ((nanos >> 4) % 7) as u32; // 120 - 126
    let chrome_patch = 6000 + ((nanos >> 6) % 500) as u32; // 6000 - 6499
    let chrome_build = 100 + ((nanos >> 8) % 100) as u32; // 100 - 199
    let chrome_ver = format!("{}.0.{}.{}", chrome_major, chrome_patch, chrome_build);

    let firefox_ver = format!("{}.0", 118 + ((nanos >> 10) % 9) as u32); // 118 - 126

    let safari_major = 605 + ((nanos >> 12) % 4) as u32; // 605 - 608
    let safari_sub_major = 15 + ((nanos >> 14) % 3) as u32; // 15 - 17
    let safari_minor = 1 + ((nanos >> 16) % 5) as u32; // 1 - 5

    let os_str = match selected_os {
        "windows" => "Windows NT 10.0; Win64; x64".to_string(),
        "mac" => "Macintosh; Intel Mac OS X 10_15_7".to_string(),
        "linux" => "X11; Linux x86_64".to_string(),
        "ios" => {
            let major = 15 + ((nanos >> 18) % 3) as u32; // 15 - 17
            let minor = (nanos >> 20) % 6;
            format!("iPhone; CPU iPhone OS {}_{} like Mac OS X", major, minor)
        }
        _ => {
            let android_ver = 10 + ((nanos >> 22) % 5) as u32; // 10 - 14
            format!("Linux; Android {}; K", android_ver)
        }
    };

    let user_agent;
    let mut sec_ch_ua = None;
    let mut sec_ch_ua_platform = None;
    let mut accept_language = "en-US,en;q=0.9".to_string();


    match selected_browser {
        "chrome" => {
            if selected_os == "ios" {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/{}.0.0.0 Mobile/15E148 Safari/605.1.15", os_str, chrome_major);
            } else if selected_os == "android" {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Mobile Safari/537.36", os_str, chrome_ver);
                sec_ch_ua = Some(format!(r#""Google Chrome";v="{}", "Chromium";v="{}", "Not.A/Brand";v="24""#, chrome_major, chrome_major));
                sec_ch_ua_platform = Some(r#""Android""#.to_string());
                accept_language = "en-US,en;q=0.9,id;q=0.8".to_string();
            } else {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36", os_str, chrome_ver);
                sec_ch_ua = Some(format!(r#""Google Chrome";v="{}", "Chromium";v="{}", "Not.A/Brand";v="24""#, chrome_major, chrome_major));
                sec_ch_ua_platform = Some(format!(r#""{}""#, if selected_os == "windows" { "Windows" } else if selected_os == "mac" { "macOS" } else { "Linux" }));
                accept_language = "en-US,en;q=0.9,id;q=0.8".to_string();
            }
        }
        "edge" => {
            let edge_major = chrome_major;
            let edge_ver = format!("{}.0.{}.{}", edge_major, 2000 + ((nanos >> 24) % 500) as u32, 50 + ((nanos >> 26) % 100) as u32);
            if selected_os == "android" {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Mobile Safari/537.36 EdgA/{}", os_str, chrome_ver, edge_ver);
                sec_ch_ua = Some(format!(r#""Microsoft Edge";v="{}", "Chromium";v="{}", "Not.A/Brand";v="24""#, edge_major, edge_major));
                sec_ch_ua_platform = Some(r#""Android""#.to_string());
            } else if selected_os == "ios" {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.0 Mobile/15E148 Safari/605.1.15 Edge/{}", os_str, edge_ver);
            } else {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36 Edg/{}", os_str, chrome_ver, edge_ver);
                sec_ch_ua = Some(format!(r#""Microsoft Edge";v="{}", "Chromium";v="{}", "Not.A/Brand";v="24""#, edge_major, edge_major));
                sec_ch_ua_platform = Some(format!(r#""{}""#, if selected_os == "windows" { "Windows" } else if selected_os == "mac" { "macOS" } else { "Linux" }));
                accept_language = "en-US,en;q=0.9,id;q=0.8".to_string();
            }
        }
        "firefox" => {
            if selected_os == "android" {
                user_agent = format!("Mozilla/5.0 (Android {}; Mobile; rv:{}) Gecko/{} Firefox/{}", 10 + ((nanos >> 28) % 5) as u32, firefox_ver, firefox_ver, firefox_ver);
                accept_language = "en-US,en;q=0.9,id;q=0.8".to_string();
            } else if selected_os == "ios" {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/{}.0 Mobile/15E148 Safari/605.1.15", os_str, firefox_ver.split('.').next().unwrap());
            } else {
                user_agent = format!("Mozilla/5.0 ({}; rv:{}) Gecko/20100101 Firefox/{}", os_str, firefox_ver, firefox_ver);
                accept_language = "en-US,en;q=0.5".to_string();
            }
        }
        _ => {
            let target_os = if selected_os == "windows" || selected_os == "linux" || selected_os == "android" {
                if (nanos >> 30) % 2 == 0 { "mac" } else { "ios" }
            } else {
                selected_os
            };
            let apple_os_str = if target_os == "ios" {
                let major = 15 + ((nanos >> 18) % 3) as u32; // 15 - 17
                let minor = (nanos >> 20) % 6;
                format!("iPhone; CPU iPhone OS {}_{} like Mac OS X", major, minor)
            } else {
                "Macintosh; Intel Mac OS X 10_15_7".to_string()
            };

            if target_os == "ios" {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/{}.1.15 (KHTML, like Gecko) Version/{}.{} Mobile/15E148 Safari/{}.1.15", apple_os_str, safari_major, safari_sub_major, safari_minor, safari_major);
            } else {
                user_agent = format!("Mozilla/5.0 ({}) AppleWebKit/{}.1.15 (KHTML, like Gecko) Version/{}.{} Safari/{}.1.15", apple_os_str, safari_major, safari_sub_major, safari_minor, safari_major);
            }
        }
    }

    BrowserProfile {
        user_agent,
        accept_language,
        sec_ch_ua,
        sec_ch_ua_platform,
    }
}
