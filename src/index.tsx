import React from 'react';
import * as ReactDOM from 'react-dom';
import ReactFlow, { Background, BackgroundVariant, MiniMap, useStoreState } from 'react-flow-renderer';

const elements = [
  { id: '1', type: 'input', data: { label: 'Node 1' }, position: { x: 250, y: 5 } },
  // you can also pass a React Node as a label
  { id: '2', data: { label: <div>Node 2</div> }, position: { x: 100, y: 100 } },
  { id: 'e1-2', source: '1', target: '2', animated: true },
];

const NodesDebugger = () => {
  const nodes = useStoreState(state => state.nodes);
  console.log(nodes);
  return null;
};

const App = () => {
  return <ReactFlow elements={elements}>
    <Background
      variant={BackgroundVariant.Dots}
      gap={16}
      size={1} />
    <MiniMap />
    <NodesDebugger />
  </ReactFlow>;
};

let element = document.createElement("div");
document.body.appendChild(element);
ReactDOM.render(React.createElement(App), element);
