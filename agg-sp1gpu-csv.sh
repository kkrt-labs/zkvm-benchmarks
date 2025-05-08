#!/bin/bash

# スクリプトの引数としてディレクトリパスを取得
TARGET_DIR=$1

# ディレクトリが指定されていない場合はエラーメッセージを表示して終了
if [ -z "$TARGET_DIR" ]; then
  echo "エラー: ディレクトリパスを指定してください。"
  echo "使用法: $0 <ディレクトリパス>"
  exit 1
fi

# ディレクトリが存在しない場合はエラーメッセージを表示して終了
if [ ! -d "$TARGET_DIR" ]; then
  echo "エラー: 指定されたディレクトリ '$TARGET_DIR' は存在しません。"
  exit 1
fi

# 処理するプレフィックスのリスト
PREFIXES=("fib" "sha2" "ethtransfer")

# 各プレフィックスについて処理を実行
for prefix in "${PREFIXES[@]}"; do
  OUTPUT_FILE="$TARGET_DIR/${prefix}_sp1-gpu.csv"
  # 出力ファイルを初期化（既存の場合は上書き）
  > "$OUTPUT_FILE"

  # 対象となるCSVファイルを見つけて自然順ソートする
  # find と sort -V を組み合わせて数値部分でソート
  # null文字区切りで処理することでファイル名にスペース等が含まれていても対応
  files_to_process=()
  while IFS= read -r -d $'\0'; do
    files_to_process+=("$REPLY")
  done < <(find "$TARGET_DIR" -maxdepth 1 -name "${prefix}_sp1-gpu-*.csv" -print0 | sort -zV)

  # 対象ファイルがない場合は次のプレフィックスへ
  if [ ${#files_to_process[@]} -eq 0 ]; then
    echo "情報: '${prefix}_sp1-gpu-*.csv' に一致するファイルは見つかりませんでした。"
    # 空の出力ファイルは削除しておく
    rm -f "$OUTPUT_FILE"
    continue
  fi

  echo "処理中: $prefix"

  # 最初のファイルからヘッダー行を読み取って出力ファイルに書き込む
  first_file="${files_to_process[0]}"
  if [ -f "$first_file" ]; then
    head -n 1 "$first_file" > "$OUTPUT_FILE"
  else
    echo "警告: 最初のファイル '$first_file' が見つかりません。ヘッダーが書き込めませんでした。"
    continue
  fi

  # 各ファイルからデータ行（ヘッダーを除く）を読み取って出力ファイルに追記
  for file in "${files_to_process[@]}"; do
    if [ -f "$file" ]; then
      # 最初のファイルはヘッダー行を既に書き込んでいるので、データ行のみ追記
      # それ以外のファイルもデータ行のみ追記
      tail -n +2 "$file" >> "$OUTPUT_FILE"
    else
      echo "警告: ファイル '$file' が見つかりません。スキップします。"
    fi
  done
  echo "完了: $OUTPUT_FILE が作成されました。"
done

echo "全ての処理が完了しました。"

