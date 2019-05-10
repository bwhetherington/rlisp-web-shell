import RLisp from "../crate/Cargo.toml";
import React from "react";
import ReactDOM from "react-dom";
import Repl from "./components/Repl";
import Code from "./components/Code";
import * as Import from "./import";
import "./index.css";

const ENTRY =
  "https://raw.githubusercontent.com/bwhetherington/rlisp2/master/rlisp-lib/loader.rl";

async function onStart() {
  const text = await Import.readFile(ENTRY);
  Import.getImports(text);
}

const code = `(define (square x)
  (* x x))
`;

function App(_) {
  return (
    <div className="page">
      <h1>RLisp Shell</h1>
      RLisp is a Lisp dialect written in Rust. It is based loosely on Scheme.
      <Code>{code}</Code>
      <div className="repl-container">
        <Repl />
      </div>
    </div>
  );
}

function load() {
  RLisp.initialize();
  onStart();
  // RLisp.set_entry_point(
  //   "https://raw.githubusercontent.com/bwhetherington/rlisp2/master/rlisp-lib/stdlib.rl"
  // );
  // fetch(
  //   "https://raw.githubusercontent.com/bwhetherington/rlisp2/master/rlisp-lib/stdlib.rl"
  // )
  //   .then(data => data.text())
  //   .then(src => {
  //     console.log(src);
  //     RLisp.handle_input(`(begin\n${src}\n)`);
  //   });
}

load();
ReactDOM.render(<App />, document.getElementById("root"));
