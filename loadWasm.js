const imports = {
  env: {
    date_now: Date.now,
  },
};
const WASM_PATH =
  "./target/wasm32-unknown-unknown/release/wasm_dev_book_hello_wasm.wasm";

const formatTimestamp = (timestamp) => {
  const date = new Date(timestamp);
  return date.toLocaleString();
};

const toUint32 = (num) => num >>> 0;

const useWasm = async () => {
  try {
    const response = await fetch(WASM_PATH);
    const bytes = await response.arrayBuffer();
    const results = await WebAssembly.instantiate(bytes, imports);

    const { add, get_timestamp, rand } = results.instance.exports;
    console.log(add(1, 2));
    console.log(formatTimestamp(get_timestamp()));
    console.log(toUint32(rand()));
  } catch (error) {
    console.error("Wasm loading and use failed:", error);
  }
};

(() => {
  useWasm();
})();
