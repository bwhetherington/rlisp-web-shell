import RL from "../crate/Cargo.toml";

export async function readFile(url) {
  try {
    const res = await fetch(url);
    const text = await res.text();
    return text;
  } catch (ex) {
    return "";
  }
}

export function runRLisp(code) {
  const res = RL.handle_input(code);
  if (res !== undefined) {
    if (res.startsWith("value:")) {
      return {
        success: true,
        type: "value",
        value: res.substring(6)
      };
    } else {
      return {
        success: false,
        type: "exception",
        value: res.substring(10)
      };
    }
  } else {
    return {
      success: false,
      value: ""
    };
  }
}

const IMPORT_REGEX = /\(\s*import\s+([^\)]+)\s*\)/g;
const ENTRY_BASE =
  "https://raw.githubusercontent.com/bwhetherington/rlisp2/master/rlisp-lib/";

function stripQuotes(str) {
  return str.replace(/"/g, "");
}

function getImportUrl(url) {
  const res = runRLisp(url);
  if (res.success) {
    url = res.value;
    if (url.startsWith("./")) {
      const file = url.substring(2);
      return `${ENTRY_BASE}${file}`;
    } else {
      return url;
    }
  } else {
    return null;
  }
}

function stripComments(text) {
  return text
    .split("\n")
    .map(line => {
      const i = line.indexOf(";");
      if (i !== -1) {
        return line.substring(0, i);
      } else {
        return line;
      }
    })
    .join("\n");
}

export function getImports(text) {
  const source = text.replace(IMPORT_REGEX, async (_, g1) => {
    const url = getImportUrl(g1);
    if (url !== null) {
      const text = await readFile(url);
      const stripped = stripComments(text);
      getImports(stripped);
    }
    return "";
  });
  const toRun = `(begin\n${source}\n)`;
  const res = runRLisp(toRun);
}

export async function importPrelude(url) {
  const text = readFile(url);
}
