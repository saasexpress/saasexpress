let Arrow = ({ length = 24, color = "#999" }: any) => {
  return (
    <svg
      viewBox={"0 0 " + length + " 10"}
      version="1.1"
      width={length}
      height="10"
      style={{ margin: "0" }}
      xmlns="http://www.w3.org/2000/svg"
    >
      <line
        x1="1"
        y1="5"
        x2={length - 3}
        y2="5"
        stroke={color}
        strokeWidth="1"
      />
      <circle cx={length - 5} cy="5" r="3" strokeWidth="0" fill={color} />
    </svg>
  );
};

export default Arrow;
