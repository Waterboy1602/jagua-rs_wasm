import { useState } from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import Header from "./components/Header.tsx";
import Home from "./components/Home.tsx";
import Input from "./components/Input.tsx";
import Solution from "./components/Solution.tsx";

import { Config } from "./interfaces/interfaces";

const defaultConfig: Config = {
    cde_config: {
        quadtree_depth: 5,
        hpg_n_cells: 2000,
        item_surrogate_config: {
            pole_coverage_goal: 0.9,
            max_poles: 10,
            n_ff_poles: 2,
            n_ff_piers: 0,
        },
    },
    poly_simpl_tolerance: 0.001,
    prng_seed: 0,
    n_samples: 5000,
    ls_frac: 0.02,
};

function App() {
    const [config, setConfig] = useState<Config>(defaultConfig);

    return (
        <Router>
            <Header config={config} setConfig={setConfig} />
            <main>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/input" element={<Input config={config} />} />
                    <Route path="/solution" element={<Solution />} />
                </Routes>
            </main>
        </Router>
    );
}

export default App;
