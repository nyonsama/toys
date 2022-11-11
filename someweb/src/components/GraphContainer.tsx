import { useRef, useEffect } from "react";
import drawGraph from "../lib/drawGraph";

interface GraphContainerProps {
  path: string;
}
const GraphContainer = ({ path, ...props }: GraphContainerProps) => {
  const graphContainer = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (graphContainer.current == null) {
      return;
    }
    const instance = drawGraph(graphContainer.current, path);
    return () => {
      instance.dispose();
    };
  });
  return (
    <div
      {...props}
      style={{ width: "100%", height: "100%" }}
      ref={graphContainer}
    />
  );
};
export default GraphContainer;
