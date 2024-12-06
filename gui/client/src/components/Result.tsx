import { useLocation } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faDownload } from "@fortawesome/free-solid-svg-icons";

import "../style/result.css";

const Result = () => {
    const location = useLocation();
    const response: string = location.state;

    const server: string = "http://localhost:8000/";

    console.log(response);

    const svgPath = response[0];
    const jsonPath = response[1][0];

    return (
        <div className="container result">
            <h1>Solution</h1>

            <a
                href={`${server}${jsonPath}`}
                download="solution.json"
                className="btn"
            >
                <FontAwesomeIcon icon={faDownload} />
                &nbsp;&nbsp;JSON
            </a>

            <div className="container solution">
                <img src={`${server}${svgPath[0]}`} />
            </div>
        </div>
    );
};

export default Result;
