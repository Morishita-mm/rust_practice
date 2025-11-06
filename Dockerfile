FROM rust:latest

# --- 開発ユーザー設定 ---
ARG USER_NAME=rustuser
ARG USER_ID=1000
ARG GROUP_ID=1000

# ユーザーの作成とパスワード設定
RUN groupadd --gid ${GROUP_ID} ${USER_NAME} \
  && useradd --uid ${USER_ID} --gid ${GROUP_ID} -m -s /bin/bash ${USER_NAME} \
  && echo "${USER_NAME}:rustpassword" | chpasswd

# root のパスワードを設定 (SSH 接続/管理用)
RUN echo 'root:root' | chpasswd

# --- SSHD 設定 ---
RUN apt-get update && apt-get install -y openssh-server

# SSHD の安定化と設定変更 (複数のRUNを一つにまとめて効率化)
RUN ssh-keygen -A && \
    sed -i 's/^#HostKey/HostKey/g' /etc/ssh/sshd_config && \
    sed -i 's/^#PermitRootLogin prohibit-password/PermitRootLogin yes/g' /etc/ssh/sshd_config && \
    echo 'PasswordAuthentication yes' >> /etc/ssh/sshd_config && \
    echo 'UsePrivilegeSeparation no' >> /etc/ssh/sshd_config && \
    echo 'UsePAM no' >> /etc/ssh/sshd_config && \
    mkdir -p /var/run/sshd

EXPOSE 22

# --- Rust ツールチェーンインストール ---
# rustuser に切り替えて rustupをインストール (これが cargo の PATH を自動設定します)
USER ${USER_NAME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

# 最終的に root で sshd を起動するため、CMD/ENTRYPOINT は compose.yaml に委ねる
