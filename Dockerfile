FROM rust:latest

ARG USER_NAME=rustuser
ARG USER_ID=1000
ARG GROUP_ID=1000

# Debian/Ubuntuベースのコマンドでユーザーを作成
RUN groupadd --gid ${GROUP_ID} ${USER_NAME} \
    && useradd --uid ${USER_ID} --gid ${GROUP_ID} -m -s /bin/bash ${USER_NAME}

# コンテナのデフォルトユーザーをrustuserに設定
USER ${USER_NAME}
