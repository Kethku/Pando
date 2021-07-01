import React, { MouseEvent, ChangeEvent, KeyboardEvent, FC, useCallback, useEffect, useState, useRef } from 'react';
import * as ReactDOM from 'react-dom';
import ReactFlow, { Background, BackgroundVariant, Edge, Node, MiniMap, OnLoadParams, Position, addEdge, Handle, isNode, useStoreState, removeElements } from 'react-flow-renderer';
import store from 'react-flow-renderer/dist/store';

function useCounter(start: number): [string, () => string] {
    const [currentId, setId] = useState(start);
    return [
        currentId.toString(),
        () => {
            setId(currentId + 1);
            return currentId.toString();
        }
    ];
}

enum TodoState {
    Pending = "Pending",
    InProgress = "InProgress",
    Done = "Done"
}

interface TodoData {
    editing: boolean,
    label: string,
    state: TodoState,
}

interface TodoParams {
    id: string, 
    data: TodoData, 
    setData: (id: string, newData: TodoData) => void,
}
const Todo = ({ id, data, setData }: TodoParams) => {
    const todoStyles = {
        border: 'solid',
        background: '#FFF',
        color: data.state == TodoState.Done ? '#999' : '#000',
        padding: 10,
    };

    const onTextChange = (event: ChangeEvent<HTMLInputElement>) => 
        setData(id, { ...data, label: event.target.value});
    const commitLabel = () => setData(id, { ...data, editing: false });
    const onKeyPress = (event: KeyboardEvent) => {
        if (event.key === "Enter") {
            commitLabel();
        }
    };

    return <div style={todoStyles}>
        <Handle type="target" position={Position.Top}/>
        {data.editing ?
            <input autoFocus type="text" value={data.label} onChange={onTextChange} onKeyPress={onKeyPress} onBlur={commitLabel} /> :
            <div>{data.label} </div>}
            {data.state == TodoState.InProgress ? <div>In Progress</div> : null}
            {data.state == TodoState.Done ? <div>Done</div> : null}
        <Handle type="source" position={Position.Bottom}/>
    </div>
};

const StateStorer = ({ flowInstance, currentId }: { flowInstance: OnLoadParams, currentId: string }) => {
    useStoreState(() => {
        if (flowInstance) {
            localStorage.setItem("data", JSON.stringify({
                elements: flowInstance.toObject().elements,
                currentId
            }));
        }
    });

    return <></>;
}

let initialElements = [];
let initialId = 0;
const storedData = localStorage.getItem("data");
if (storedData != null) {
    let { elements, currentId } = JSON.parse(storedData);
    initialElements = elements;
    initialId = currentId;
}

const App = () => {
    const reactFlowWrapper = useRef(null);
    const [flowInstance, setFlowInstance] = useState<OnLoadParams>(null);
    const [elements, setElements] = useState(initialElements);
    const [currentId, getId] = useCounter(initialId);

    const setData = (id: string, newData: TodoData) => {
        setElements((elements) => {
            var elementsWithoutNode = elements.filter(node => node.id != id || !isNode(node));
            var nodeToUpdate = elements.find(node => node.id == id && isNode(node));
            nodeToUpdate.data = newData;
            return elementsWithoutNode.concat(nodeToUpdate);
        });
    };

    const onPaneClick = (event: MouseEvent<Element, globalThis.MouseEvent>) => {
      event.preventDefault();

      const reactFlowBounds = reactFlowWrapper.current.getBoundingClientRect();
      const position = flowInstance.project({
          x: event.clientX - reactFlowBounds.left,
          y: event.clientY - reactFlowBounds.top,
      });
      const id = getId();
      const newNode = {
          id,
          type: "todo",
          position,
          data: { label: "", editing: true, state: TodoState.Pending },
      };

      setElements((elements) => elements.concat(newNode));
    };

    const onNodeDoubleClick = (_event: any, node: Node<TodoData>) => {
        var todoState = node.data.state;
        switch (todoState) {
            case TodoState.Done: 
                todoState = TodoState.Pending
                break;
            case TodoState.InProgress: 
                todoState = TodoState.Done;
                break;
            case TodoState.Pending: 
                todoState = TodoState.InProgress;
                break;
        }
        setData(node.id, {...node.data, state: todoState});
    };

    const onNodeContextMenu = (event: MouseEvent<Element, globalThis.MouseEvent>, node: Node<TodoData>) => {
        event.preventDefault();
        setElements((elements) => removeElements([node], elements));
    };

    const onConnect = (newEdge: Edge) => setElements((elements) => addEdge(newEdge, elements));

    const WrappedTodo = ({ id, data }: { id: string, data: TodoData }) => {
        return Todo({id, data, setData});
    };

    const nodeTypes = {
        todo: WrappedTodo
    };

    return <div className="reactflow-wrapper" ref={reactFlowWrapper}>
        <ReactFlow 
            elements={elements}
            onPaneClick={onPaneClick}
            onNodeDoubleClick={onNodeDoubleClick}
            onNodeContextMenu={onNodeContextMenu}
            onConnect={onConnect}
            onLoad={setFlowInstance}
            nodeTypes={nodeTypes}>
            <Background
                variant={BackgroundVariant.Dots}
                gap={16}
                size={1} />
            <MiniMap />
            <StateStorer flowInstance={flowInstance} currentId={currentId} />
        </ReactFlow>
    </div>;
};

let element = document.createElement("div");
document.body.appendChild(element);
ReactDOM.render(React.createElement(App), element);
