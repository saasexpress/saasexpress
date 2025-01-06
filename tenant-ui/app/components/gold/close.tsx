const Close = ({ size }: any) => {
  const svgSize = size ? size : 20;

  return (
    <div
      className="svg-close"
      style={{
        verticalAlign: "middle",
        display: "inline-block",
        width: svgSize,
        height: svgSize,
        opacity: 1,
      }}
    >
      <svg
        viewBox="0 0 24 24"
        version="1.1"
        preserveAspectRatio="xMinYMin"
        width="100%"
        height="100%"
        xmlns="http://www.w3.org/2000/svg"
      >
        <line x1="1" y1="23" x2="23" y2="1" stroke="black" strokeWidth="1" />
        <line x1="1" y1="1" x2="23" y2="23" stroke="black" strokeWidth="1" />
      </svg>
    </div>
  );
};

export default Close;
