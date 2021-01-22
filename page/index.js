import * as wasm from "largefile-sha256-bench";
import { memory } from "largefile-sha256-bench/bigfile_sha265_bench_bg";

import CryptoJS from "crypto-js";
const file = document.getElementById("file");
file.addEventListener("change", e => {
  let files = e.target.files[0];
  //生成实例
  let fileReads = new FileReader();
  //开始读取文件
  fileReads.readAsArrayBuffer(files);
  //读取回调
  fileReads.onload = function () {
    let sha256Struct = wasm.Sha256Struct.new();
    let MAX_BUFFER_LENGTH = 1024 * 1024
    const array = new Uint8Array(memory.buffer, sha256Struct.buffer_ptr(), 1024 * 1024);
    //将读取结果：文件数据类型：ArrayBuffer 转化 为wordArray格式
    let buffer = new Uint8Array(fileReads.result);
    console.time("wasm-sha2");
    for (let i = 0; i < 100; i++) {
      let j = 0;
      while (j < buffer.length) {
        let len = Math.min(buffer.length - j, MAX_BUFFER_LENGTH);
        array.set(buffer.subarray(j, j + len), 0);
        sha256Struct.update(len);
        j += len;
      }
      sha256Struct.clear()
      // sha2Result = wasm.sha265(buffer)
    }
    console.timeEnd("wasm-sha2");
    // console.log(res)

    let ccresult;
    console.time("wasm-c");
    for (let i = 0; i < 100; i++) {
      ccresult = wasm.sha256(buffer);
    }
    console.timeEnd("wasm-c");
    let encodeResult = new Uint8Array(memory.buffer, ccresult, 64);
    let decoder = new TextDecoder().decode(encodeResult);
    console.log(decoder);
    console.time("js");
    for (let i = 0; i < 100; i++) {
      // console.time("loop js")
      var wordArray = CryptoJS.lib.WordArray.create(fileReads.result);
      //直接调用SHA256()并转化得到十六进制字符串（也就是我们要的SHA256）
      var hash = CryptoJS.SHA256(wordArray).toString();
      // console.timeEnd("loop js")
    }
    console.timeEnd("js");
    console.log(hash);
  };
});
