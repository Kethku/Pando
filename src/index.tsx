import React, { MouseEvent, useState } from 'react';
import * as ReactDOM from 'react-dom';
import ReactFlow, { Background, BackgroundVariant, MiniMap, OnLoadParams, useRef, useStoreState } from 'react-flow-renderer';

function useCounter(start: number) {
  const [currentId, setId] = useState();

}

const Todo = ({ data }) => {
};

const App = () => {
  const reactFlowWrapper = useRef(null);
  const [flowInstance, setFlowInstance] = useState<OnLoadParams>(null);
  const [elements, setElements] = useState([]);

  const OnPaneClick = (event: MouseEvent<Element, globalThis.MouseEvent>) => {
    event.preventDefault();

    const reactFlowBounds = reactFlowWrapper.current.getBoundingClientRect();
    const position = flowInstance.project({
      x: event.clientX - reactFlowBounds.left,
      y: event.clientY - reactFlowBounds.top,
    });
    const newNode = {
      id: getId(),
      type,
      position,
      data: { label: `${type} node` },
    };

    setElements((es) => es.concat(newNode));
  };

  return <div className="reactflow-wrapper" ref={reactFlowWrapper}>
    <ReactFlow 
      nodeTypes={{ todo: Todo }}
      elements={elements}
      onPaneClick={OnPaneClick}
      onLoad={setFlowInstance}>
      <Background
        variant={BackgroundVariant.Dots}
        gap={16}
        size={1} />
      <MiniMap />
    </ReactFlow>
  </div>;
};

let element = document.createElement("div");
document.body.appendChild(element);
ReactDOM.render(React.createElement(App), element);
