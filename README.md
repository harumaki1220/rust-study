# Rust 学習リポジトリ (rust-study)

このリポジトリはRustの学習用プロジェクトを管理します。

## 
## 
## 新規プロジェクトの追加手順

1.  `rust-study` フォルダのルートで、`--vcs none` オプションを付けて新しいプロジェクトを作成します。
    （これにより、プロジェクト内に `.git` フォルダが作られるのを防ぎます）

    ```bash
    cargo new [プロジェクト名] --vcs none
    ```
    *(例: `cargo new variables_study --vcs none`)*

2.  新しいプロジェクトをGitの管理対象に追加します。

    ```bash
    git add .
    ```

3.  変更をコミット（記録）します。
    メッセージは日本語で分かりやすく記述します。

    ```bash
    git commit -m "[プロジェクト名]のプロジェクトを追加"
    ```
    *(例: `git commit -m "variables_studyのプロジェクトを追加"`)*

4.  変更をGitHubにプッシュ（アップロード）します。

    ```bash
    git push
    ```
