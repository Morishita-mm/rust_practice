# 競技プログラミング Rust 学習記録

## 共通ルール・設定
- **入力**: `proconio::input!` マクロを使用する。
- **出力**: `println!` マクロ、または `print!` マクロを使用する。

## 学習した項目

### 1. 基本的な入出力 (Ch01 - The First Problem)
- **C++**: `cin >> N; cout << N * N << endl;`
- **Rust**:
  ```rust
  input! { n: i32 }
  println!("{}", n * n);
  ```
- **ポイント**:
  - `input!` 内で変数名と型を指定する。
  - `println!` の `{}` はフォーマット指定子。

### 2. 数列の探索と早期リターン (Ch01 - Find X)
- **C++**: ループと `return`。
- **Rust (ループと `return`)**:
  ```rust
  for i in 0..n {
      if a[i] == x {
          println!("{}", i + 1);
          return;
      }
  }
  ```
- **Rust (イテレータの活用)**:
  ```rust
  if let Some(pos) = a.iter().position(|&val| val == x) {
      println!("{}", pos + 1);
  }
  ```
- **ポイント**:
  - `return;` で `main` を抜けるのが最も手軽。
  - `Iterator::position` を使うと、ループを簡潔に書ける。

### 3. イテレータによる探索と二重ループ (Ch01 - Two Cards)
- **C++**: `for` 文の二重ループと早期 `return`。
- **Rust (イテレータ)**:
  ```rust
  let exists = a.iter().any(|&ai| b.iter().any(|&bj| ai + bj == k));
  ```
- **ポイント**:
  - `any(|x| ...)` は、条件を満たす要素が見つかると即座に（ショートサーキット） `true` を返す。
  - 二重ループも `any` をネストさせることで、簡潔かつ安全に記述できる。
### 7. ビット全探索 (Ch01 - Subset Sum / Bit Full Search)
- **C++**: `for (int i = 0; i < (1 << N); i++)`, `(i / wari) % 2 == 1`
- **Rust**:
  ```rust
  for i in 0..(1 << n) {
      let mut sum = 0;
      for j in 0..n {
          if (i >> j) & 1 == 1 {
              sum += a[j];
          }
      }
      if sum == k { exists = true; break; }
  }
  ```
- **ポイント**:
  - `1 << n` で $2^n$ を表す（$n$ は `usize` 推奨）。
  - `(i >> j) & 1 == 1` は「$i$ の $j$ 番目のビットが立っているか」を確認する定石。
  - すべての「選ぶ・選ばない」の組み合わせを網羅する際に非常に強力。

