import React from "react";
import RLisp from "../../crate/Cargo.toml";
import { runRLisp } from "../import";
import "./Repl.css";

function renderValue(value, id) {
  if (value !== undefined) {
    switch (value.type) {
      case "value":
        return <div key={id}>{value.value}</div>;
      case "command":
        return (
          <div key={id} className="command">
            {value.value}
          </div>
        );
      case "exception":
        return (
          <div key={id} className="exception">
            {value.value}
          </div>
        );
    }
  } else {
    return <div key={id} />;
  }
}

class Repl extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      history: [],
      historyIndex: -1,
      messages: [],
      input: ""
    };

    this.onChangeInput = event => {
      this.setState({
        ...this.state,
        input: event.target.value.substring(2)
      });
    };

    this.messagesEnd = React.createRef();
    this.input = React.createRef();

    this.onRun = event => {
      event.preventDefault();
      const { input } = this.state;
      const output = runRLisp(input);
      const cmd = {
        success: true,
        type: "command",
        value: `> ${input}`
      };

      let messages;
      if (output.value !== "()") {
        messages = [...this.state.messages, cmd, output];
      } else {
        messages = [...this.state.messages, cmd];
      }
      const history = [input, ...this.state.history];
      this.setState({
        history,
        historyIndex: history.length - 1,
        messages,
        input: ""
      });
    };
  }

  componentDidUpdate() {
    this.messagesEnd.current.scrollIntoView();
  }

  render() {
    let id = 0;
    return (
      <div className="repl">
        <div className="history">
          {this.state.messages.map(line => renderValue(line, id++))}
          <form onSubmit={this.onRun} className="input">
            <input
              spellCheck={false}
              type="text"
              onChange={this.onChangeInput}
              value={`> ${this.state.input}`}
              ref={this.input}
            />
          </form>
          <div ref={this.messagesEnd} />
        </div>
      </div>
    );
  }
}

export default Repl;
