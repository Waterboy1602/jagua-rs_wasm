import "../style/input.css";

interface ResultProps {
    data: string;
}

const Result = (data?: ResultProps) => {
    console.log(data);

    return (
        <div>
            <h1>Solution</h1>

            <a href={data?.data} download="solution.json" className="btn">
                <i className="fa fa-download"></i>
                JSON
            </a>

            <div className="container solution">
                <img src={data?.data} />
            </div>
        </div>
    );
};

export default Result;
