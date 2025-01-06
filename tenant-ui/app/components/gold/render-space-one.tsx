import { useState } from "react";

const RenderSpaceOne = () => {
  const [lines, setLines] = useState({ lines: [{}] });

  const _lines = lines.lines.map((l) => {
    let w = Math.round(Math.random() * 100);
    let h = 25;
    let style = {
      width: "" + w + "%",
      height: "" + h + "px",
      backgroundColor: "#EFEFEF",
      marginBottom: "5px",
    };
    return <div style={style} key={"w" + w}></div>;
  });

  let key = "fake";
  return (
    <div className="fake-space" key={key}>
      {_lines}
    </div>
  );
};

export default RenderSpaceOne;
