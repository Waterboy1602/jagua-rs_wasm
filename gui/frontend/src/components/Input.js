// gui/frontend/src/components/Input.js
import React from "react";
import axios from "axios";
import { useLocation } from "react-router-dom";

import '../style/input.css';

// Handle form submission
const handleSubmit = (event, jsonInput, selected, setResponseMessage, setError) => {

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
}

const parseJson = (jsonInput, selected) => {
    const parsedJson = JSON.parse(jsonInput);
    var items = parsedJson.Items;
    var selectedItems = [];

    for (var i = 0; i < items.length; i++) {
        if (selected[i]) {
            selectedItems.push(items[i]);
        }
    }

    parsedJson.Items = selectedItems;
    return parsedJson;
}

const renderShape = (shape) => (
    <div>
      <b>Data:</b>
      <ul>
        {shape.Data.map((point, index) => (
          <li key={index}>
            ({point[0]}, {point[1]})
          </li>
        ))}
      </ul>
    </div>
  );

  const renderItems = (items, selected) => {      
      return items.map((item, index) => (
        <div key={index} className="item">
          <h3>Item {index + 1}</h3>
          <p>
            <b>Select:</b> 
            <input className="boolean" type="checkbox" value={selected[index]} />
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
                  item.AllowedOrientations.map((orientation, index) => (
                      <>
                          <input className="number" type="number" value={orientation} />
                      </>
                  ))
              }
              <br/>
          </p>
          <div>
            {renderShape(item.Shape)}
          </div>
        </div>
      ));
  };



const Input = () => {
    const location = useLocation();
    const jsonData = location.state?.jsonData;
    var selected = [];

    return (
        <div className="container input">
            <h1 className="title">JSON Overview - {jsonData.Name}</h1>

            <div className="container items">
                {renderItems(jsonData.Items, selected)}
            </div>

            <button className="submit" type="submit" onClick={handleSubmit}>Submit</button>
        </div>

    );
};

export default Input;