import React from "react";
import "./Code.css";

function Code(props) {
  return (
    <pre className="Code">
      <code>{props.children}</code>
    </pre>
  );
}

export default Code;
