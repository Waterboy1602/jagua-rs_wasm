import { useState } from "react";
import { useNavigate } from "react-router-dom";

import styles from "../styles/Home.module.css";

interface ParsedJson {
    [key: string]: unknown;
}

interface NavigateState {
    input: ParsedJson;
}

const Home: React.FC = () => {
    const [jsonInput, setJsonInput] = useState<string>(""); // Holds the input JSON string
    const [file, setFile] = useState<File | null>(null);
    const [error, setError] = useState<string | null>(null); // To display any errors
    const navigate = useNavigate();

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
        event.preventDefault(); // Prevent default form submission

        if (file) {
            const reader = new FileReader();

            reader.onload = (e) => {
                try {
                    const inputFile = e.target?.result as string;
                    const parsedInput: ParsedJson = JSON.parse(inputFile);
                    console.log(parsedInput);

                    // Navigate after successful parsing
                    setError(null);
                    navigate("/input", {
                        state: { input: parsedInput } as NavigateState,
                    });
                } catch {
                    setError("Invalid JSON file. Please correct it.");
                }
            };

            reader.onerror = () => {
                setError("Error reading the file. Please try again.");
            };

            reader.readAsText(file);
        } else if (jsonInput !== "") {
            try {
                const parsedJson: ParsedJson = JSON.parse(jsonInput);
                setError(null);

                navigate("/input", {
                    state: { input: parsedJson } as NavigateState,
                });
            } catch {
                setError("Invalid JSON input. Please correct it.");
            }
        } else {
            setError("Please provide a JSON file or input.");
        }
    };

    interface FileChangeEvent extends React.ChangeEvent<HTMLInputElement> {
        target: HTMLInputElement & { files: FileList };
    }

    const handleFileChange = (event: FileChangeEvent): void => {
        const uploadedFile: File | null = event.target.files[0];
        if (uploadedFile && uploadedFile.type !== "application/json") {
            setError("Please upload a valid JSON file.");
            setFile(null);
            return;
        }

        setFile(uploadedFile);
        setError(null);
    };

    return (
        <div className={styles.home}>
            <div className={styles.forms}>
                <div>
                    <form onSubmit={handleSubmit} className={styles.form}>
                        <label htmlFor="json_str" className={styles.label}>
                            JSON text input
                        </label>
                        <textarea
                            id="json_str"
                            name="json_str"
                            placeholder="Enter your JSON here..."
                            value={jsonInput}
                            onChange={(e) => setJsonInput(e.target.value)}
                            className={styles.textarea}
                        />

                        <button type="submit" className={styles.button}>
                            Submit
                        </button>
                    </form>

                    {error && <p style={{ color: "red" }}>{error}</p>}
                </div>

                <div>
                    <form onSubmit={handleSubmit} className={styles.form}>
                        <label htmlFor="json_str" className={styles.label}>
                            JSON file input
                        </label>
                        <input
                            type="file"
                            accept="application/json"
                            onChange={handleFileChange}
                            className={styles.inputFile}
                        />

                        <button type="submit" className={styles.button}>
                            Submit
                        </button>
                    </form>

                    {error && <p style={{ color: "red" }}>{error}</p>}
                </div>
            </div>
        </div>
    );
};

export default Home;
