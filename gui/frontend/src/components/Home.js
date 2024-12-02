import React, { useState } from "react";
import axios from "axios";

import '../style/index.css';

// Handle form submission
const handleSubmit = (event, jsonInput, setResponseMessage, setError) => {
    event.preventDefault(); // Prevent default form submission

    try {
      // Attempt to parse JSON to validate it before sending
      JSON.parse(jsonInput);
    } catch (err) {
      setError("Invalid JSON input. Please correct it.");
      return;
    }

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



const Home = () => {
    const [jsonInput, setJsonInput] = useState(""); // Holds the input JSON string
    const [responseMessage, setResponseMessage] = useState(""); // To display the response
    const [error, setError] = useState(null); // To display any errors

    return (
        <div className="App">
            <h1>Jagua-rs</h1>

            {/* Form */}
            <form onSubmit={(e) => handleSubmit(e, jsonInput, setResponseMessage, setError)}>
                <label htmlFor="json_str">JSON</label>
                <br/>
                <textarea
                    id="json_str"
                    name="json_str"
                    placeholder="Enter your JSON here..."
                    value={jsonInput}
                    onChange={(e) => setJsonInput(e.target.value)}
                ></textarea>

                <br/><br/>
                <button type="submit">Submit</button>
                </form>
        </div>
    );
}

export default Home;