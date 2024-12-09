import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Header from "./components/Header.tsx";
import Home from "./components/Home.tsx";
import Input from "./components/Input.tsx";
import Result from "./components/Result.tsx";

function App() {
    return (
        <Router>
            <Header />
            <main>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/input" element={<Input />} />
                    <Route path="/result" element={<Result />} />
                </Routes>
            </main>
        </Router>
    );
}

export default App;
