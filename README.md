# branchman

ローカルGitリポジトリのブランチを表示・管理するTUIアプリケーション

## 機能

- ローカルブランチの一覧表示（更新日時順）
- 複数ブランチの選択と一括削除
- vim風のキーバインド
- 保護ブランチ（main, master, 現在のブランチ）の削除防止

## インストール

### 前提条件

- Git
- Rust (cargo)

Rustがインストールされていない場合:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### ビルド

```bash
./build.sh
```

または直接:
```bash
cargo build --release
```

## 使用方法

```bash
# 現在のディレクトリで実行
./target/release/branchman

# 特定のディレクトリを指定
./target/release/branchman /path/to/git/repo

# シンボリックリンクを作成して使用
ln -s $(pwd)/target/release/branchman ./app
./app ./
```

## キーバインド

### 通常モード
- `j` / `↓` : カーソルを下へ
- `k` / `↑` : カーソルを上へ
- `Space` : ブランチの選択/選択解除
- `d` : 選択したブランチを削除（確認ダイアログ表示）
- `g` : リストの先頭へジャンプ
- `G` : リストの末尾へジャンプ
- `q` : アプリケーション終了

### 削除確認ダイアログ
- `y` / `Enter` : 削除を実行
- `n` / `Esc` : キャンセル

## 保護ブランチ

以下のブランチは削除できません:
- `main`
- `master`
- 現在チェックアウトしているブランチ（★マーク付き）