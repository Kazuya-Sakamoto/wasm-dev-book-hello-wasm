// npx http-server .

const imports = {
  env: {
    date_now: Date.now,
  },
};

const wasm =
  "./target/wasm32-unknown-unknown/release/wasm_dev_book_hello_wasm.wasm";

fetch(wasm)
  .then((response) => response.arrayBuffer())
  .then((bytes) => WebAssembly.instantiate(bytes, imports))
  .then((results) => {
    const { add, get_timestamp } = results.instance.exports;
    console.log(add(1, 2));
    // 追加
    console.log(get_timestamp());
  });
