import * as wasm from "./pkg/wasm_dev_book_hello_wasm.js";

const HOBBIES = {
  DEVELOPMENT: "開発",
  MUSIC: "音楽",
  SPORTS: "スポーツ",
  TRAVEL: "旅行",
  READING: "読書",
  GAMING: "ゲーム",
  ANIME: "アニメ",
};

const formatTimestamp = (timestamp) => {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString();
};

const toUint32 = (num) => num >>> 0;

const useWasmFunctions = () => {
  console.log(wasm.add(1, 2));
  console.log(formatTimestamp(wasm.get_timestamp()));
  console.log(toUint32(wasm.rand()));
  console.log(
    `String length: ${wasm.get_string_length("Hello, WebAssembly!")}`
  );
};

const fetchAndProcessData = async () => {
  try {
    const response = await fetch(
      "https://jsonplaceholder.typicode.com/todos/1"
    );
    const json = await response.json();
    console.log("Original JSON:", json);

    const modifiedJson = wasm.modify_response(json.title, json.completed);
    console.log("Modified JSON:", modifiedJson);
  } catch (error) {
    console.error("Error fetching and processing data:", error);
  }
};

const processUserInput = (inputText) => {
  console.log(`Word count: ${wasm.count_words(inputText)}`);
};

const onSearhUser = () => {
  const param = {
    name: "ka",
    sex: 1,
    age: 28,
    hobby: HOBBIES.DEVELOPMENT,
  };

  console.log(
    wasm.search_by_criteria(param.name, param.sex, param.age, param.hobby)
  );
};

const main = async () => {
  await wasm.default();
  useWasmFunctions();
  await fetchAndProcessData();
  processUserInput("This is a sample text for WebAssembly.");
  onSearhUser();
};

main().catch(console.error);
