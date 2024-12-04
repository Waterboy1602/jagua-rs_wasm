// gui/frontend/src/components/Input.js
import axios from "axios";
import { useLocation } from "react-router-dom";

import '../style/input.css';

// Handle form submission
interface HandleSubmitProps {
    event: React.MouseEvent<HTMLButtonElement>;
    jsonInput: string;
    selected: boolean[];
    setResponseMessage: React.Dispatch<React.SetStateAction<string>>;
    setError: React.Dispatch<React.SetStateAction<string | null>>;
}

const handleSubmit = ({ event, jsonInput, selected, setResponseMessage, setError }: HandleSubmitProps) => {
    event.preventDefault();

    parseJson(jsonInput, selected);

    setError(null); // Clear any existing error
    setResponseMessage("Loading...");

    // Make the POST request
    axios
        .post("http://localhost:8000/json", { json_str: jsonInput })
        .then((response) => {
            setResponseMessage(response.data.message); // Assuming backend returns a `message`
        })
        .catch((err) => {
            setError("Failed to fetch data from the backend.");
            console.error(err);
        });
};

interface ParsedJson {
    Items: Item[];
}

const parseJson = (jsonInput: string, selected: boolean[]): ParsedJson => {
    const parsedJson: ParsedJson = JSON.parse(jsonInput);
    const items: Item[] = parsedJson.Items;
    const selectedItems: Item[] = [];

    for (let i = 0; i < items.length; i++) {
        if (selected[i]) {
            selectedItems.push(items[i]);
        }
    }

    parsedJson.Items = selectedItems;
    return parsedJson;
}

interface Shape {
    Data: boolean[][];
}

interface Item {
    Demand: number;
    DemandMax: number;
    AllowedOrientations: number[];
    Shape: Shape;
}


const renderShape = (shape: Shape) => (
  <div>
    <b>Data:</b>
    <ul>
      {shape.Data.map((point: boolean[], index: number) => (
        <li key={index}>
          ({point[0]}, {point[1]})
        </li>
      ))}
    </ul>
  </div>
);



  const renderItems = (items: Item[], selected: boolean[]) => {      
    return items.map((item, index: number) => (
        <>
        <div key={index} className="item">
            <h3>Item {index + 1}</h3>

            <p>
                <b>Select:</b> 
                <input className="boolean" type="checkbox" checked={selected[index]} />
            </p>

            <p>
                <b>Demand:</b> 
                <input className="number" type="number" value={item.Demand} />
            </p>

            <p>
                <b>Demand Max:</b> 
                <input className="number" type="number" value={item.DemandMax} />
            </p>

            <p>
                <b>Allowed Orientations:</b>
                {
                    item.AllowedOrientations.map((orientation: number, idx: number) => (
                        <input key={idx} className="number" type="number" value={orientation} />
                    ))
                }
                <br/>
            </p>

            <div>
                {renderShape(item.Shape)}
            </div>
        </div>
        </>
    ));
  };



const Input = () => {
    const location = useLocation();
    const jsonData = location.state?.jsonData;
    const selected: boolean[] = [];

    return (
        <div className="container input">
            <h1 className="title">JSON Overview - {jsonData.Name}</h1>

            <div className="container items">
                {renderItems(jsonData.Items, selected)}
            </div>

            <button className="submit" type="submit" onClick={(event) => handleSubmit({
                event,
                jsonInput: JSON.stringify(jsonData),
                selected,
                setResponseMessage: () => {},
                setError: () => {}
            })}>Submit</button>
        </div>

    );
};

export default Input;