import { useState } from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import Header from "./components/Header.tsx";
import Home from "./components/Home.tsx";
import Input from "./components/Input.tsx";
import Result from "./components/Result.tsx";

import { Config } from "./interfaces/interfaces";

function App() {
    const [config, setConfig] = useState<Config>({
        quadtreeDepth: 5,
        hpgNCells: 2000,
        poleCoverageGoal: 90,
        maxPoles: 10,
        nFFPoles: 2,
        nFFPiers: 0,
        polySimplTolerance: 0.1,
        prngSeed: 0,
        nSamples: 5000,
        lsFrac: 2,
    });

    return (
        <Router>
            <Header config={config} setConfig={setConfig} />
            <main>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/input" element={<Input config={config} />} />
                    <Route path="/result" element={<Result />} />
                </Routes>
            </main>
        </Router>
    );
}

export default App;
