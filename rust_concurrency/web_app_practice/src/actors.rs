use std::{collections::HashMap, time::Duration};

use crate::models::VoteRecord;
use tokio::{sync::{broadcast, mpsc}, time::Instant};

// アクターが受け取るメッセージの型
pub enum VoteMessage {
    NewVote {
        team_name: String,
        current_count: i32,
    },
}

// アクター本体
pub struct VoteObserverActor {
    // メールボックス（受信機）
    receiver: mpsc::Receiver<VoteMessage>,
    broadcaster: broadcast::Sender<VoteRecord>,
    history: HashMap<String, Vec<Instant>>,
}

impl VoteObserverActor {
    // アクターの作成（メールボックスと送信機を返す）
    pub fn new(broadcaster: broadcast::Sender<VoteRecord>) -> (Self, VoteObserverHandle) {
        let (tx, rx) = mpsc::channel(32); // 容量32のメールボックス
        let actor = Self {
            receiver: rx,
            broadcaster,
            history: HashMap::new(),
        };
        let handle = VoteObserverHandle { sender: tx };
        (actor, handle)
    }

    pub async fn run(mut self) {
        // メッセージが来るたびに処理を行う
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                VoteMessage::NewVote {
                    team_name,
                    current_count,
                } => {
                    if self.is_spam(&team_name) {
                        println!("⚠️ SPAM DETECTED: {}チームへの投票が多すぎます。通知をスキップします", team_name);
                        continue;
                    }
                    println!(
                        "Actor: {}チームに票が入りました。（現在{}票)",
                        team_name, current_count
                    );

                    let record = VoteRecord {
                        team_name: team_name.clone(),
                        count: current_count,
                    };
                    self.broadcaster.send(record).ok();

                    // 特定のロジック（イベント駆動）
                    if current_count >= 10 {
                        println!("🎉 Actor: {}チームが10票突破！お祝いしましょう", team_name);
                    }
                }
            }
        }
    }
    fn is_spam(&mut self, team_name: &str) -> bool {
        let now = Instant::now();
        let window = Duration::from_secs(10);
        let limit = 5;

        let timestamps = self.history.entry(team_name.to_string()).or_insert(Vec::new());

        timestamps.push(now);

        timestamps.retain(|&t| now.duration_since(t) < window);

        timestamps.len() > limit
    }
}

#[derive(Clone)]
pub struct VoteObserverHandle {
    sender: mpsc::Sender<VoteMessage>,
}

impl VoteObserverHandle {
    // 外部からメッセージを送るためのメソッド
    pub async fn notify_new_vote(&self, team_name: String, count: i32) {
        let msg = VoteMessage::NewVote {
            team_name,
            current_count: count,
        };
        // エラー（アクターが死んでる場合）は今回は無視
        let _ = self.sender.send(msg).await;
    }
}
