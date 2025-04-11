import React, {
  Component,
  JSX,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import {
  Stage,
  Layer,
  Rect,
  Line,
  Text,
  Group,
  Circle,
  Tag,
  Label,
  Image,
} from "react-konva";
import Konva from "konva";
import Code from "components/gold/code";
import { KonvaEventObject } from "konva/lib/Node";
import { Stack } from "@mui/material";
import { Vector2d } from "konva/lib/types";
import useAPIClient from "lib/api/APIClient";
import { useQueryClient } from "@tanstack/react-query";
import APIErrorHandler from "lib/alerts/APIErrorHandler";
import {
  DAGVariant,
  GraphNodeHover,
  Service,
  Edge,
  Node,
} from "components/graph";
import { stringify } from "yaml";

interface DynamicGraphProps {
  id: string;
  variant: string;
  data: Service;
  onSelected: Function;
}

const DynamicGraph: React.FC<DynamicGraphProps> = ({
  id,
  data,
  variant,
  onSelected,
}: DynamicGraphProps) => {
  const api = useAPIClient();
  const queryClient = useQueryClient();

  const stageRef = useRef<Konva.Stage>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  const [mouseOver, setMouseOver] = useState<{
    node: string;
    over?: boolean;
    selected?: boolean;
  }>({
    node: "",
    over: false,
    selected: false,
  });
  const actionBarRef = useRef(null);

  const [stagePosition, setStagePosition] = useState({ x: 0, y: 0 });
  const [stageScale, setStageScale] = useState(1);
  const [stageSize, setStageSize] = useState({ width: 0, height: 500 });

  const GRID_SIZE = 20; // Base grid size

  useEffect(() => {
    if (data && variant) {
      const dagVariant: DAGVariant = data.variants[variant];
      console.log(dagVariant);
      dagVariant.dag.nodes.map((nd, idx) => {
        nd.x = 100 + idx * 120;
        nd.y = 100;
        nd.label = nd.id;
      });
      setEdges(dagVariant.dag.edges);
      setNodes(dagVariant.dag.nodes);

      if (dagVariant.dag.visuals) {
        const visuals = JSON.parse(dagVariant.dag.visuals);
        setStagePosition(visuals.stagePosition);
        setStageScale(visuals.stageScale);
        dagVariant.dag.nodes.map((nd) => {
          visuals.nodes
            .filter((ndviz: any) => nd.id === ndviz.id)
            .map((ndviz: any) => {
              nd.x = ndviz.x;
              nd.y = ndviz.y;
            });
        });
      }
    }
  }, [data, variant]);

  // useEffect(() => {
  //   updateVisuals(stagePosition, stageScale, nodes);
  // }, [stageScale]);

  // Graph data: nodes and edges
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  const updateVisuals = (
    stagePosition: any,
    stageScale: any,
    nodes: Node[]
  ) => {
    const visuals = {
      stagePosition,
      stageScale,
      nodes: nodes.map((node) => ({
        id: node.id,
        x: node.x,
        y: node.y,
      })),
    };

    const dagVariant: DAGVariant = data.variants[variant];
    dagVariant.dag.visuals = JSON.stringify(visuals);

    api.put(`/api/services/${id}`, data, () => {
      queryClient.invalidateQueries({ queryKey: ["service", id] });
      queryClient.invalidateQueries({ queryKey: ["services"] });
      queryClient.invalidateQueries({ queryKey: ["list-activity"] });
      APIErrorHandler.notice({
        title: "Service",
        content: "Updated visuals",
      });
    });
  };

  // Helper function to find a node by ID
  const findNode = (id: string): Node | undefined =>
    nodes.find((node) => node.id === id);

  const setWithBounds = (value: number, min: number, max: number) => {
    if (value < min) return min;
    if (value > max) return max;
    return Math.round(value);
  };

  const handleDragEnd = (e: Konva.KonvaEventObject<DragEvent>, id: string) => {
    handleDragMove(e, id);
    updateVisuals(stagePosition, stageScale, nodes);
  };

  // Drag handler for nodes
  const handleDragMove = (e: Konva.KonvaEventObject<DragEvent>, id: string) => {
    const newNodes = nodes.map((node) =>
      node.id === id
        ? {
            ...node,
            x: setWithBounds(
              e.target.x() + 40,
              -stagePosition.x + 50,
              -stagePosition.x + (stageSize.width - 50) / stageScale
            ),
            y: setWithBounds(
              e.target.y() + 40,
              -stagePosition.y + 50,
              stageSize.height - 50
            ),
          }
        : node
    );
    setNodes(newNodes);
  };

  // Function to update stage size based on container's clientWidth
  const updateStageSize = () => {
    if (containerRef.current) {
      const containerWidth = containerRef.current.clientWidth - 10;
      const containerHeight = containerWidth * 0.45;
      setStageSize({
        width: containerWidth,
        height: containerHeight,
      });
    }
  };

  // Update stage size on window resize
  useEffect(() => {
    updateStageSize();

    window.addEventListener("resize", updateStageSize);

    // Clean up the event listener on unmount
    return () => window.removeEventListener("resize", updateStageSize);
  }, []);

  // Handle stage drag start
  const handleStageDragStart = (e: any) => {
    // Prevent the stage from dragging when the rectangle is being dragged
    if (e.target === stageRef.current) {
      e.cancelBubble = true; // Prevent propagation to stage
    }
  };

  // Add a new node
  const addNode = () => {
    const newNodeId = `Node${nodes.length + 1}`;
    const newNode: Node = {
      id: newNodeId,
      x: nodes.length > 0 ? nodes[nodes.length - 1].x + 200 : 100,
      y: nodes.length > 0 ? nodes[nodes.length - 1].y : 100,
      label: newNodeId,
    };
    setNodes([...nodes, newNode]);

    if (nodes.length === 0) return;
    const newEdge: Edge = { from: nodes[nodes.length - 1].id, to: newNodeId };
    setEdges([...edges, newEdge]);
  };

  // Add an edge between two nodes
  const addEdge = () => {
    if (nodes.length < 2) return;
    const from = nodes[nodes.length - 2].id; // Second last node
    const to = nodes[nodes.length - 1].id; // Last node
    const newEdge: Edge = { from, to };
    setEdges([...edges, newEdge]);
  };

  // Remove the last node
  const removeNode = () => {
    if (nodes.length === 0) return;
    const lastNodeId = nodes[nodes.length - 1].id;
    setNodes(nodes.slice(0, -1));
    setEdges(
      edges.filter((edge) => edge.from !== lastNodeId && edge.to !== lastNodeId)
    );
  };

  // Draw visible grid lines
  const drawGrid = useCallback((): JSX.Element[] => {
    const lines: JSX.Element[] = [];
    const scaleGridSize = GRID_SIZE * stageScale; // Adjust grid size based on zoom
    const stage = stageRef.current;
    if (!stage) return lines;

    const stageX = stagePosition.x;
    const stageY = stagePosition.y;

    // Calculate the visible range
    const startX = Math.floor(-stageX / scaleGridSize) * scaleGridSize;
    const endX =
      Math.ceil((-stageX + stageSize.width / stageScale) / scaleGridSize) *
      scaleGridSize;
    const startY = Math.floor(-stageY / scaleGridSize) * scaleGridSize;
    const endY =
      Math.ceil((-stageY + stageSize.height / stageScale) / scaleGridSize) *
      scaleGridSize;

    // Draw vertical lines
    for (let x = startX; x <= endX; x += scaleGridSize) {
      lines.push(
        <Line
          key={`v-${x}`}
          points={[x, startY, x, endY]}
          stroke="#ddd"
          strokeWidth={1}
        />
      );
    }

    // Draw horizontal lines
    for (let y = startY; y <= endY; y += scaleGridSize) {
      lines.push(
        <Line
          key={`h-${y}`}
          points={[startX, y, endX, y]}
          stroke="#ddd"
          strokeWidth={1}
        />
      );
    }
    return lines;
  }, [stageRef, stagePosition, stageSize, stageScale]);

  // Zoom handler
  const handleZoom = useCallback(
    (factor: number) => {
      const stage = stageRef.current;
      if (!stage) return;

      const oldScale = stage.scaleX();
      const newScale =
        Math.round(
          (Math.max(0.4, oldScale * factor) + Number.EPSILON) * 10000
        ) / 10000; // Prevent over-zooming
      const pointerPos: Vector2d =
        stage.getPointerPosition() == null
          ? { x: 0, y: 0 }
          : (stage.getPointerPosition() as Vector2d);

      const mousePointTo = {
        x: pointerPos.x / oldScale - stage.x() / oldScale,
        y: pointerPos.y / oldScale - stage.y() / oldScale,
      };

      setStageScale(newScale);

      const newPosition = {
        x: Math.round(-(mousePointTo.x - pointerPos.x / newScale) * newScale),
        y: Math.round(-(mousePointTo.y - pointerPos.y / newScale) * newScale),
      };

      setStagePosition(newPosition);

      updateVisuals(newPosition, newScale, nodes);
    },
    [stageRef, nodes]
  );

  // Pan handler
  const handlePan = (dx: number, dy: number) => {
    setStagePosition((prev) => ({
      x: Math.round(prev.x + dx),
      y: Math.round(prev.y + dy),
    }));
  };

  // Center stage
  const handleCenter = () => {
    setStagePosition({ x: 0, y: 0 });
    setStageScale(1);
    updateVisuals({ x: 0, y: 0 }, 1, nodes);
  };

  const [lastTouchDistance, setLastTouchDistance] = useState(0);

  // Function to calculate distance between two touch points
  const getTouchDistance = (touches: TouchList) => {
    const [touch1, touch2] = [touches[0], touches[1]];
    const dx = touch2.pageX - touch1.pageX;
    const dy = touch2.pageY - touch1.pageY;
    return Math.sqrt(dx * dx + dy * dy);
  };

  // Handle touch start (two fingers)
  const handleTouchStart = (e: Konva.KonvaEventObject<TouchEvent>) => {
    if (e.evt.touches.length === 2) {
      setLastTouchDistance(getTouchDistance(e.evt.touches));
    }
  };

  // Handle touch move (zooming)
  const handleTouchMove = (e: Konva.KonvaEventObject<TouchEvent>) => {
    if (e.evt.touches.length === 2 && stageRef.current) {
      const newTouchDistance = getTouchDistance(e.evt.touches);
      if (lastTouchDistance === 0) return;

      // Prevent the default browser pinch zoom behavior
      e.evt.preventDefault();

      // Calculate scale factor
      const scaleBy = newTouchDistance / lastTouchDistance;

      // Adjust stage scale
      const newScale = Math.max(0.1, Math.min(10, stageScale * scaleBy));

      // Calculate the position of the zoom
      const stage = stageRef.current;
      const mousePointTo = {
        x: stage.getPointerPosition()!.x / stageScale - stage.x() / stageScale,
        y: stage.getPointerPosition()!.y / stageScale - stage.y() / stageScale,
      };

      // Set new scale and position
      setStageScale(newScale);
      setStagePosition({
        x: Math.round(
          -(mousePointTo.x - stage.getPointerPosition()!.x / newScale) *
            newScale
        ),
        y: Math.round(
          -(mousePointTo.y - stage.getPointerPosition()!.y / newScale) *
            newScale
        ),
      });

      // Update last touch distance for next move
      setLastTouchDistance(newTouchDistance);
    }
  };

  // Reset touch distance on touch end
  const handleTouchEnd = () => {
    setLastTouchDistance(0);
  };

  const handleOnWheel = useCallback(
    (e: any) => {
      // stop default scrolling
      e.evt.preventDefault();

      const stage = stageRef.current;
      if (!stage) return;

      const oldScale = stage.scaleX();
      const pointer = stage.getPointerPosition();

      // Calculate scale factor
      const scaleBy = e.evt.deltaY < 0 ? 1.025 : 0.975;

      // Adjust stage scale
      const newScale = Math.max(0.1, Math.min(10, oldScale * scaleBy));

      // Calculate the position of the zoom
      const mousePointTo = {
        x: pointer!.x / oldScale - stage.x() / oldScale,
        y: pointer!.y / oldScale - stage.y() / oldScale,
      };

      // Set new scale and position
      setStageScale(newScale);
      setStagePosition({
        x: Math.round(-(mousePointTo.x - pointer!.x / newScale) * newScale),
        y: Math.round(-(mousePointTo.y - pointer!.y / newScale) * newScale),
      });
    },
    [stageRef]
  );

  const handleMouseLeave = (e: any) => {
    //const relatedTarget = e.relatedTarget;
    // Check if the relatedTarget (new hovered element) is inside the action bar
    if (
      actionBarRef.current &&
      (actionBarRef.current as any)
        .getStage()
        .findOne(`#action-bar`)
        ?.hasPointerCapture(e.pointerId)
    ) {
      return;
    }
    setMouseOver({ node: mouseOver.node, selected: mouseOver.selected });
  };

  return (
    <div>
      <div
        ref={containerRef}
        style={{
          width: "100%",
          height: "100%",
          border: "1px solid #ddd",
          marginBottom: "10px",
          marginTop: "10px",
          minHeight: "500",
        }}
      >
        <Stage
          width={stageSize.width}
          height={stageSize.height}
          scaleX={stageScale}
          scaleY={stageScale}
          x={stagePosition.x}
          y={stagePosition.y}
          // onWheel={handleOnWheel}
          // onTouchStart={handleTouchStart}
          // onTouchMove={handleTouchMove}
          // onTouchEnd={handleTouchEnd}
          // onTouchCancel={handleTouchEnd}
          draggable
          onDragMove={(e) => {
            if (e.target === stageRef.current) {
              setStagePosition({
                x: Math.round(e.target.x()),
                y: Math.round(e.target.y()),
              });
            }
          }}
          onDragStart={handleStageDragStart}
          onDragEnd={() => {
            updateVisuals(stagePosition, stageScale, nodes);
          }}
          ref={stageRef}
        >
          <Layer>{drawGrid()}</Layer>
          <Layer>
            {/* Render edges */}
            {edges.map((edge, index) => {
              const fromNode = findNode(edge.from);
              const toNode = findNode(edge.to);

              if (!fromNode || !toNode) return <></>; // Handle missing nodes

              return (
                <Line
                  key={index}
                  points={[
                    fromNode.x + 20,
                    fromNode.y,
                    toNode.x + 20,
                    toNode.y,
                  ]}
                  stroke="black"
                  strokeWidth={3}
                />
              );
            })}

            {/* Render nodes */}
            {nodes.map((node) => (
              <React.Fragment key={node.id}>
                <GraphNodeHover
                  nodes={nodes}
                  node={node}
                  hover={mouseOver.node === node.id && mouseOver.over}
                  onAction={() =>
                    setMouseOver({
                      node: node.id,
                      over: false,
                      selected: mouseOver.selected,
                    })
                  }
                />
                <Rect
                  x={node.x - 40}
                  y={node.y - 40}
                  width={120}
                  height={80}
                  fill="#00D2FF"
                  strokeWidth={
                    mouseOver.node === node.id && mouseOver.selected ? 3 : 0
                  } // border width
                  stroke="blue" // border color
                  cornerRadius={3}
                />
                <Text
                  x={node.x - 35}
                  y={node.y - 70}
                  width={120}
                  height={80}
                  align="left"
                  verticalAlign="middle"
                  text={node.action}
                  fontSize={12}
                  fill="black"
                />
                <Text
                  x={node.x - 40}
                  y={node.y - 40}
                  width={120}
                  height={80}
                  align="center"
                  verticalAlign="middle"
                  text={node.label}
                  fontSize={16}
                  fill="black"
                  onClick={() => {
                    onSelected(node);
                    setMouseOver({
                      ...mouseOver,
                      ...{ node: node.id, selected: !mouseOver.selected },
                    });
                  }}
                  onMouseOver={(e) =>
                    setMouseOver({
                      node: node.id,
                      over: true,
                      selected: mouseOver.selected,
                    })
                  }
                  onMouseOut={handleMouseLeave}
                  draggable
                  onDragStart={handleStageDragStart}
                  onDragMove={(e) => handleDragMove(e, node.id)}
                  onDragEnd={(e) => handleDragEnd(e, node.id)}
                />
              </React.Fragment>
            ))}
          </Layer>
        </Stage>
      </div>
      <Stack direction="row" style={{ marginBottom: "10px" }} spacing={1}>
        <button onClick={addNode} style={{}}>
          Add Node
        </button>
        <button onClick={addEdge} style={{}}>
          Add Edge
        </button>
        <button onClick={removeNode}>Remove Node</button>
        <button onClick={() => handleZoom(1.2)}>Zoom In</button>
        <button onClick={() => handleZoom(0.8)}>Zoom Out</button>
        <button onClick={() => handlePan(-50, 0)}>Left</button>
        <button onClick={() => handlePan(50, 0)}>Right</button>
        <button onClick={() => handlePan(0, -50)}>Up</button>
        <button onClick={() => handlePan(0, 50)}>Down</button>
        <button onClick={handleCenter}>Center</button>
      </Stack>
      <Code>{stringify(nodes, null, 5)}</Code>
      {/* <Code>
        {JSON.stringify(stageSize)}
        {JSON.stringify(stagePosition)}
        {JSON.stringify(stageScale)}
        {JSON.stringify({
          nodes: nodes.map((node) => ({
            id: node.id,
            x: node.x,
            y: node.y,
          })),
          edges,
        })}
      </Code> */}
    </div>
  );
};

export default DynamicGraph;
