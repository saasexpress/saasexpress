export interface Service {
  displayName: string;
  variants: Record<string, DAGVariant>;
}

export interface DAGVariant {
  dag: DAG;
}

export interface DAG {
  edges: Edge[];
  nodes: Node[];
  name: string;
  visuals: any;
}

export interface Node {
  id: string;
  x: number;
  y: number;
  label: string;
  action?: string;
  config?: any;
}

export interface Edge {
  from: string;
  to: string;
}
