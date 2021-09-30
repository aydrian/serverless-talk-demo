import React from "react";
import { render } from "react-dom";

function App() {
  return <div>Hello {window.username || "World"}</div>;
}

render(<App />, document.getElementById("app"));
