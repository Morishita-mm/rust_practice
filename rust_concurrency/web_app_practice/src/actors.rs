use tokio::sync::mpsc;

// アクターが受け取るメッセージの型
pub enum VoteMessage {
    NewVote { team_name: String, current_count: i32 },
}

// アクター本体
pub struct VoteObserverActor {
    // メールボックス（受信機）
    receiver: mpsc::Receiver<VoteMessage>,
}

impl VoteObserverActor {
    // アクターの作成（メールボックスと送信機を返す）
    pub fn new() -> (Self, VoteObserverHandle) {
        let (tx, rx) = mpsc::channel(32);   // 容量32のメールボックス
        let actor = Self { receiver: rx };
        let handle = VoteObserverHandle { sender: tx };
        (actor, handle)
    }

    pub async fn run(mut self) {
        // メッセージが来るたびに処理を行う
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                VoteMessage::NewVote { team_name, current_count } => {
                    println!("Actor: {}チームに票が入りました。（現在{}票)", team_name, current_count);

                    // 特定のロジック（イベント駆動）
                    if current_count >= 10 {
                        println!("🎉 Actor: {}チームが10票突破！お祝いしましょう", team_name);
                    }
                }
            }
        }
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
            current_count: count
        };
        // エラー（アクターが死んでる場合）は今回は無視
        let _ = self.sender.send(msg).await;
    }
}