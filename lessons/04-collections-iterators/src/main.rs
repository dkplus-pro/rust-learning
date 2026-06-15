// Lesson 04: 集合、迭代器与数据处理管道 demo
// 运行：cargo run -p lesson_04_collections_iterators
//
// 前端类比：把迭代器理解为更惰性、更类型安全的 array pipeline。

use std::collections::HashMap;

#[derive(Debug)]
struct PageView {
    route: &'static str,
    user: &'static str,
    ms_spent: u32,
}

fn main() {
    let views = sample_page_views();

    let active_users = unique_active_users(&views);
    println!("活跃用户数：{}", active_users.len());
    println!("活跃用户：{}", active_users.join(", "));

    let route_totals = total_time_by_route(&views);
    println!("\n各路由停留时间：");
    for (route, total_ms) in &route_totals {
        println!("  {route:<12} {total_ms:>5}ms");
    }

    match top_route(&route_totals) {
        Some((route, total_ms)) => println!("\n最值得优化的路由：{route}，累计 {total_ms}ms"),
        None => println!("没有页面访问数据"),
    }

    let slow_views: Vec<&PageView> = views.iter().filter(|view| view.ms_spent >= 900).collect();
    println!("\n慢访问记录：");
    for view in slow_views {
        println!("  {} 在 {} 停留 {}ms", view.user, view.route, view.ms_spent);
    }
}

fn sample_page_views() -> Vec<PageView> {
    vec![
        PageView {
            route: "/dashboard",
            user: "alice",
            ms_spent: 820,
        },
        PageView {
            route: "/settings",
            user: "bob",
            ms_spent: 420,
        },
        PageView {
            route: "/dashboard",
            user: "carol",
            ms_spent: 1_240,
        },
        PageView {
            route: "/reports",
            user: "alice",
            ms_spent: 960,
        },
        PageView {
            route: "/reports",
            user: "dave",
            ms_spent: 1_100,
        },
    ]
}

fn unique_active_users(views: &[PageView]) -> Vec<String> {
    let mut users = views
        .iter()
        .map(|view| view.user.to_string())
        .collect::<Vec<_>>();
    users.sort();
    users.dedup();
    users
}

fn total_time_by_route(views: &[PageView]) -> HashMap<&'static str, u32> {
    views.iter().fold(HashMap::new(), |mut totals, view| {
        let total = totals.entry(view.route).or_insert(0);
        *total += view.ms_spent;
        totals
    })
}

fn top_route(totals: &HashMap<&'static str, u32>) -> Option<(&'static str, u32)> {
    totals
        .iter()
        .max_by_key(|(_, total_ms)| *total_ms)
        .map(|(route, total_ms)| (*route, *total_ms))
}
