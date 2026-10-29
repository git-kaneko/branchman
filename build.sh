#!/bin/bash

# Rustのインストール確認
if ! command -v cargo &> /dev/null; then
    echo "Rustがインストールされていません。"
    echo "以下のコマンドでインストールしてください:"
    echo ""
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "インストール後、以下を実行してください:"
    echo "source \$HOME/.cargo/env"
    exit 1
fi

# ビルド実行
echo "ビルドを実行中..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "ビルド成功！"
    echo "実行ファイル: ./target/release/branchman"
    echo ""
    echo "使用方法:"
    echo "  ./target/release/branchman [ディレクトリパス]"
    echo ""
    echo "または、シンボリックリンクを作成:"
    echo "  ln -s $(pwd)/target/release/branchman ./app"
    echo "  ./app [ディレクトリパス]"
else
    echo "ビルド失敗"
    exit 1
fi