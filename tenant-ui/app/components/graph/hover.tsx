import { Circle, Group, Label, Line, Rect, Tag, Text } from "react-konva";
import { Node } from "./types";
import { useState } from "react";
import { KonvaEventObject } from "konva/lib/Node";

interface GraphNodeHoverProps {
  node: Node;
  hover: boolean | undefined;
  onAction: Function;
  nodes: Node[];
}

const SNAP_RADIUS = 20;

const GraphNodeHover = ({
  nodes,
  node,
  hover,
  onAction,
}: GraphNodeHoverProps) => {
  const [lines, setLines] = useState([] as any);
  const [draggingNode, setDraggingNode] = useState(null as any);
  const [tempLine, setTempLine] = useState(null as any);

  const handleDragStart = (nodeId: string) => {
    const node = nodes.find((n: Node) => n.id === nodeId);
    if (node) {
      setDraggingNode(node);
      setTempLine({ x1: node.x, y1: node.y, x2: node.x, y2: node.y });
    }
  };

  const handleDragMove = (e: any) => {
    if (!draggingNode || !tempLine) return;

    setTempLine((prev: any) => ({
      ...prev,
      x2: e.target.x(),
      y2: e.target.y(),
    }));
  };

  const handleDragEnd = (nodeId: string, e: any) => {
    if (!draggingNode || !tempLine) return;

    const { x: endX, y: endY } = e.target.position();

    // Find the closest node within SNAP_RADIUS
    const targetNode = nodes
      .filter((n) => n.id !== nodeId) // Exclude the starting node
      .reduce((closest: any, node: Node) => {
        const dist = Math.hypot(node.x - endX, node.y - endY);
        return dist < (closest?.dist ?? Infinity) ? { node, dist } : closest;
      }, null)?.node;

    if (
      targetNode &&
      Math.hypot(targetNode.x - endX, targetNode.y - endY) <= SNAP_RADIUS
    ) {
      // Snap to nearest node
      setLines([
        ...lines,
        {
          x1: tempLine.x1,
          y1: tempLine.y1,
          x2: targetNode.x,
          y2: targetNode.y,
        },
      ]);
    }

    setTempLine(null);
    setDraggingNode(null);
  };

  return (
    <Group id="action-bar" visible={hover}>
      <Rect
        x={node.x - 60}
        y={node.y - 60}
        width={160}
        height={120}
        fill="#EFEFEF"
        onMouseOut={() => onAction()}
      />
      {/* <Circle
        x={node.x + 80 + 10}
        y={node.y - 0}
        radius={5}
        fill="white"
        stroke="black"
        strokeWidth={2}
        onMouseOver={() => false}
      />
      <Circle
        x={node.x - 40 - 10}
        y={node.y - 0}
        radius={5}
        fill="white"
        stroke="black"
        strokeWidth={2}
        onMouseOver={() => false}
      />
      <Circle
        x={node.x + 20}
        y={node.y - 40 - 10}
        radius={5}
        fill="white"
        stroke="black"
        strokeWidth={2}
        onMouseOver={() => false}
      /> */}
      {/* <Circle
        x={node.x + 20}
        y={node.y + 40 + 10}
        radius={5}
        fill="white"
        stroke="black"
        strokeWidth={2}
        onMouseOver={() => true}
        draggable
        //        onMouseOver={(e: KonvaEventObject<MouseEvent>) => e.cancelBubble}
        onDragStart={() => handleDragStart(node.id)}
        onDragMove={handleDragMove}
        onDragEnd={(e) => handleDragEnd(node.id, e)}
      /> */}
      <Label
        x={node.x + 80}
        y={node.y - 60}
        width={150}
        onClick={() => alert("Clicked!")}
      >
        <Tag fill="black" lineJoin="round"></Tag>
        <Text text="X" fontSize={14} padding={2} fill="white"></Text>
      </Label>
      {/* Temporary Line (while dragging) */}
      {/* Render Lines */}
      {lines.map((line: any, index: any) => (
        <Line
          key={index}
          points={[line.x1, line.y1, line.x2, line.y2]}
          stroke="black"
          strokeWidth={2}
        />
      ))}
      {tempLine && (
        <Line
          points={[tempLine.x1, tempLine.y1, tempLine.x2, tempLine.y2]}
          stroke="red"
          strokeWidth={2}
          dash={[5, 5]}
        />
      )}
    </Group>
  );
};

export default GraphNodeHover;
