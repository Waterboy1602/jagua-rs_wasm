import { useState, Dispatch, SetStateAction, useEffect, useRef } from "react";
import { useNavigate } from "react-router-dom";
import styles from "../styles/Home.module.css";
import { Status } from "../Enums";

interface HomeProps {
  svgResult: string | null;
  setSvgResult: Dispatch<SetStateAction<string | null>>;
  logs: string[];
  setLogs: Dispatch<SetStateAction<string[]>>;
}
interface ParsedJson {
  [key: string]: unknown;
}

interface NavigateState {
  input: ParsedJson;
}

interface FileChangeEvent extends React.ChangeEvent<HTMLInputElement> {
  target: HTMLInputElement & { files: FileList };
}

function Home({ svgResult, setSvgResult, logs, setLogs }: HomeProps) {
  const [file, setFile] = useState<File | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();
  const [useDemoFile, setUseDemoFile] = useState(true);

  const workerRef = useRef<Worker | null>(null);

  useEffect(() => {
    workerRef.current = new Worker(new URL("../services/wasmWorker.ts", import.meta.url), {
      type: "module",
    });

    workerRef.current.onmessage = (event) => {
      const { type, message, result } = event.data;

      if (type === Status.FINISHED) {
        setSvgResult(result);
        setLogs((prevLogs) => [...prevLogs, `Finished`]);
        setLoading(false);
      } else if (type === Status.ERROR) {
        setLogs((prevLogs) => [...prevLogs, `Error: ${message}`]);
      } else {
        setLogs((prevLogs) => [...prevLogs, message]);
      }
    };

    return () => {
      if (workerRef.current) {
        workerRef.current.terminate();
        workerRef.current = null;
      }
    };
  }, []);

  const startWasmComputation = (svgInput: string): void => {
    setLogs([]);
    if (workerRef.current) {
      workerRef.current.postMessage({
        type: Status.START,
        payload: { svgInput: svgInput },
      });
    }
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>): Promise<void> => {
    event.preventDefault();
    setLoading(true);
    setError(null);

    if (useDemoFile) {
      try {
        const response = await fetch("swim.svg");
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const fileContent = await response.text();
        startWasmComputation(fileContent);
      } catch (e) {
        setError("Failed to load demo file: " + e);
      }

      return;
    }

    if (file) {
      const reader = new FileReader();

      reader.onload = async (e) => {
        const fileContent = e.target?.result as string;

        try {
          if (file.type === "image/svg+xml") {
            startWasmComputation(fileContent);
          }

          if (file.type === "application/json") {
            try {
              const parsedInput: ParsedJson = JSON.parse(fileContent);

              navigate("/input", {
                state: { input: parsedInput } as NavigateState,
              });
            } catch {
              setError("Invalid JSON file. Please correct it.");
            }
          }
        } catch (wasmError) {
          setError("WASM processing error: " + wasmError);
        }
      };

      reader.onerror = () => {
        setError("Error reading the file. Please try again.");
        setLoading(false);
      };

      reader.readAsText(file);
    } else {
      setError("Please upload a file.");
      setLoading(false);
    }
  };

  const handleFileChange = (event: FileChangeEvent): void => {
    const uploadedFile: File | null = event.target.files[0];
    if (uploadedFile) {
      if (uploadedFile.type === "image/svg+xml" || uploadedFile.type === "application/json") {
        setFile(uploadedFile);
        setError(null);
      } else {
        setError("Please upload a valid SVG or JSON file.");
        setFile(null);
      }
    } else {
      setFile(null);
      setError(null);
    }
  };

  const handleUseDemoChange = (event: React.ChangeEvent<HTMLInputElement>): void => {
    if (event.target.checked) {
      setUseDemoFile(true);
    } else {
      setUseDemoFile(false);
    }
  };

  if (!svgResult) {
    return (
      <div className={styles.home}>
        <div className={styles.forms}>
          <div>
            <form onSubmit={(event) => handleSubmit(event)} className={styles.form}>
              <label className={styles.label}>Upload an input file</label>
              <label className={styles.subLabel}>SVG or JSON</label>
              <input
                type="file"
                accept="image/svg+xml, application/json"
                onChange={handleFileChange}
                className={styles.inputFile}
              />

              <label className={styles.demoCheckboxWrapper}>
                <input
                  type="checkbox"
                  defaultChecked={useDemoFile}
                  onChange={handleUseDemoChange}
                  className={styles.demoCheckbox}
                />
                <span className={styles.demoLabel}>Use demo file</span>
              </label>

              <button type="submit" className={styles.button} disabled={loading}>
                {loading ? <span className={styles.loader} /> : "Submit"}
              </button>
            </form>

            {error && <p style={{ color: "red" }}>{error}</p>}
          </div>
        </div>

        {logs.length > 0 && (
          <div className={styles.logBox}>
            <h4>Logs</h4>
            <ul>
              {logs.map((log, idx) => (
                <li key={idx}>{log}</li>
              ))}
            </ul>
          </div>
        )}
      </div>
    );
  } else {
    return (
      <div
        dangerouslySetInnerHTML={{ __html: svgResult }}
        style={{ width: "100%", height: "100%" }}
      />
    );
  }
}

export default Home;
