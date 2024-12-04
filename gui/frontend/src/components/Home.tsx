import { useState } from "react";
import { useNavigate } from "react-router-dom";

import '../style/index.css';

interface ParsedJson {
    [key: string]: unknown;
}

interface NavigateState {
    jsonData: ParsedJson;
}

const Home: React.FC = () => {
    const [jsonInput, setJsonInput] = useState<string>(""); // Holds the input JSON string
    const [error, setError] = useState<string | null>(null); // To display any errors
    const navigate = useNavigate();

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
        event.preventDefault(); // Prevent default form submission
    
        try {
            // Attempt to parse JSON to validate it before sending
            const parsedJson: ParsedJson = JSON.parse(jsonInput);
      
            // Clear any existing errors
            setError(null);
      
            // Navigate to "/input" with validated JSON data
            navigate("/input", { state: { jsonData: parsedJson } as NavigateState });
        } catch {
            // Show error message if JSON is invalid
            setError("Invalid JSON input. Please correct it.");
        }
    };

    return (
        <div className="home">
            <h1>Jagua-rs</h1>

            {/* Form */}
            <form onSubmit={handleSubmit}>
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

            {/* Error Message */}
            {error && <p style={{ color: "red" }}>{error}</p>}
        </div>
    );
}

export default Home;