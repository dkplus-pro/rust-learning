// Lesson 07: 并发、线程与消息传递 demo
// 运行：cargo run -p lesson_07_concurrency
//
// 前端类比：像把构建任务分发给多个 worker，主线程只接收完成消息。
// Rust 的类型系统会阻止多个线程同时不安全地修改同一块数据。

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
struct AssetJob {
    name: &'static str,
    size_kb: u64,
}

#[derive(Debug)]
struct AssetReport {
    name: &'static str,
    duration_ms: u64,
}

fn main() {
    let jobs = vec![
        AssetJob {
            name: "app.js",
            size_kb: 420,
        },
        AssetJob {
            name: "vendor.js",
            size_kb: 860,
        },
        AssetJob {
            name: "styles.css",
            size_kb: 120,
        },
        AssetJob {
            name: "dashboard.wasm",
            size_kb: 1_200,
        },
    ];

    let (sender, receiver) = mpsc::channel::<AssetReport>();
    let mut handles = Vec::new();

    for job in jobs {
        let worker_sender = sender.clone();
        let handle = thread::spawn(move || {
            let duration_ms = simulate_build(job);
            let report = AssetReport {
                name: job.name,
                duration_ms,
            };
            worker_sender
                .send(report)
                .expect("主线程应该在 worker 完成前保持接收端存活");
        });
        handles.push(handle);
    }

    // 关闭主线程持有的 sender。所有 worker 结束后，receiver 的 for 循环会自然停止。
    drop(sender);

    let mut reports = Vec::new();
    for report in receiver {
        println!(
            "收到构建结果：{} 用时 {}ms",
            report.name, report.duration_ms
        );
        reports.push(report);
    }

    for handle in handles {
        handle.join().expect("worker 不应该 panic");
    }

    reports.sort_by_key(|report| report.duration_ms);
    let total_ms: u64 = reports.iter().map(|report| report.duration_ms).sum();

    println!("\n按耗时排序：");
    for report in &reports {
        println!("  {:<16} {}ms", report.name, report.duration_ms);
    }
    println!("总模拟耗时：{total_ms}ms（真实墙钟时间更短，因为任务并发执行）");
}

fn simulate_build(job: AssetJob) -> u64 {
    // 用文件大小模拟构建耗时，sleep 只是为了让并发效果可观察。
    let duration_ms = 40 + job.size_kb / 20;
    thread::sleep(Duration::from_millis(duration_ms));
    duration_ms
}
