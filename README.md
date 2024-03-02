# wasm-dev-book-hello-wasm

WebAssembly の検証ができるプロジェクトです

## 実行

- build

```
$ wasm-pack build --target web
```

- run server

```
$ npx http-server .
```

## WebAssembly とは何か

WebAssembly は基本的なブラウザで動作し、新たな機能と大幅なパフォーマンス向上を提供する新しい種類のコードです。基本的に直接記述ではなく、C、C++、Rust 等の**低水準（アセンブリ言語）の言語にとって効果的なコンパイル対象**となるように設計されている。**コンパイル済みのバイナリコード**であるため、ブラウザが解釈・実行する際に、JavaScript よりも高速に動作する。

**ブラウザ上で動作するクライアントアプリで従来は実現できなかった、ネイティブ水準の速度で複数の言語で記述されたコードをブラウザ上で動作させる方法を提供**します。

MDN 引用: https://developer.mozilla.org/ja/docs/WebAssembly

### 参考になった記事

- [WebAssembly 開発環境構築の本](https://wasm-dev-book.netlify.app/) を参考に検証
- [Rust から WebAssembly にコンパイル](https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_Wasm)
- 誕生の経緯は[WebAssembly の歴史について](https://zenn.dev/hodagi/articles/4925afbeb3c4dc)がまとまっていてよかった 👍
