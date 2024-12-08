import { useState } from "react";
import axios from "axios";
import { useLocation, useNavigate } from "react-router-dom";

import "../style/input.css";

interface Shape {
    Type: string;
    Data: number[][];
}

interface Item {
    Demand: number;
    DemandMax: number;
    AllowedOrientations: number[];
    Shape: Shape;
}

const makeJSON = (
    name: string,
    items: Item[],
    selected: boolean[],
    strip: string
): string => {
    const jsonObj: { Name?: string; Items?: Item[]; Strip?: string } = {
        Name: "",
        Items: [],
    };

    jsonObj["Name"] = name;
    jsonObj["Items"] = [];

    for (let i = 0; i < selected.length; i++) {
        if (selected[i]) {
            jsonObj["Items"].push(items[i]);
        }
    }
    jsonObj["Strip"] = strip;

    return JSON.stringify(jsonObj);
};

const Input = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const jsonData = location.state?.jsonData;
    const [items, setItems] = useState<Item[]>(jsonData.Items);
    const [selected, setSelected] = useState<boolean[]>(
        new Array(jsonData.Items.length).fill(true)
    );

    const handleSubmit = () => {
        const json: string = makeJSON(
            jsonData.Name,
            items,
            selected,
            jsonData.Strip
        );

        console.log(json);

        axios
            .post("http://localhost:8000/json", { json_str: json })
            .then((response) => {
                console.log(response);
                navigate("/result", { state: response.data });
            })
            .catch((err) => {
                console.error(err);
            });
    };

    const handleCheckboxChange = (index: number) => {
        const newSelected = [...selected];
        newSelected[index] = !newSelected[index];
        setSelected(newSelected);
    };

    const SvgComponent = ({ shape }: { shape: Shape }) => {
        const maxX = Math.max(...shape.Data.map((p) => p[0]));
        const maxY = Math.max(...shape.Data.map((p) => p[1]));
        const points = shape.Data.map((p) => `${p[0]},${p[1]}`).join(" ");

        return (
            <div className="svg-container">
                <b>Shape:</b>
                <svg
                    viewBox={`-50 -50 ${maxX + 100} ${maxY + 100}`}
                    preserveAspectRatio="xMidYMid meet"
                >
                    <polyline
                        points={points}
                        fill="none"
                        stroke="black"
                        strokeWidth="2"
                        vector-effect="non-scaling-stroke"
                    />
                </svg>
            </div>
        );
    };

    const renderItems = (items: Item[], selected: boolean[]) => {
        return items.map((item, index: number) => (
            <>
                <div key={index} className="item">
                    <div>
                        <input
                            className="boolean"
                            type="checkbox"
                            checked={selected[index]}
                            onChange={() => handleCheckboxChange(index)}
                        />

                        <h3>Item {index + 1}</h3>
                    </div>

                    <hr />

                    <p>
                        <b>Demand:</b>
                        <input
                            className="number"
                            type="number"
                            value={item.Demand}
                            onChange={(e) => {
                                const newItems = [...items];
                                newItems[index].Demand = parseInt(
                                    e.target.value
                                );
                                setItems(newItems);
                            }}
                        />
                    </p>

                    <p>
                        <b>Demand Max:</b>
                        <input
                            className="number"
                            type="number"
                            value={item.DemandMax}
                            onChange={(e) => {
                                const newItems = [...items];
                                newItems[index].DemandMax = parseInt(
                                    e.target.value
                                );
                                setItems(newItems);
                            }}
                        />
                    </p>

                    <p>
                        <b>Allowed Orientations:</b>
                        {item.AllowedOrientations.map(
                            (orientation: number, idx: number) => (
                                <input
                                    key={idx}
                                    className="number"
                                    type="number"
                                    value={orientation}
                                    onChange={(e) => {
                                        const newItems = [...items];
                                        newItems[index].AllowedOrientations[
                                            index
                                        ] = parseInt(e.target.value);
                                        setItems(newItems);
                                    }}
                                />
                            )
                        )}
                        <br />
                    </p>

                    <SvgComponent shape={item.Shape} />
                </div>
            </>
        ));
    };

    return (
        <div className="container input">
            <div className="title">
                <h1>JSON Overview - {jsonData.Name}</h1>

                <button className="submit" type="submit" onClick={handleSubmit}>
                    Submit
                </button>
            </div>

            <div className="container items">
                {renderItems(jsonData.Items, selected)}
            </div>
        </div>
    );
};

export default Input;
