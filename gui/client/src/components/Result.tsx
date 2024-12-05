import { useLocation } from "react-router-dom";

import "../style/input.css";

// interface ResponseProps {
//     data: string[]; // Array containing file paths
// }

const Result = () => {
    const location = useLocation();
    const response: string = location.state;

    const server: string = "http://localhost:8080";

    console.log(response);

    const svgPath = response[0];
    const jsonPath = response[1];

    return (
        <div>
            <h1>Solution</h1>

            <a
                href={`${server}${jsonPath}`}
                download="solution.json"
                className="btn"
            >
                <i className="fa fa-download"></i>
                JSON
            </a>

            <div className="container solution">
                <img src={`${server}${svgPath}`} />
            </div>
        </div>
    );
};

export default Result;
