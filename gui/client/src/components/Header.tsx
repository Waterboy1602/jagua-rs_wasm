import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faGear } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";
import { Config } from "../interfaces/interfaces";

import styles from "../styles/Header.module.css";

interface HeaderProps {
    config: Config;
    setConfig: React.Dispatch<React.SetStateAction<Config>>;
}

const Header: React.FC<HeaderProps> = ({ config, setConfig }) => {
    const [showSettings, setShowSettings] = useState(false);

    const toggleConfig = () => {
        setShowSettings(!showSettings);
    };

    const closeConfig = () => {
        setShowSettings(false);
    };

    const saveConfig = () => {
        // Add logic to save settings here
        const form = document.querySelector("form");
        if (form) {
            const formData = new FormData(form);
            const newConfig: Config = {
                quadtreeDepth: Number(formData.get("quadtreeDepth")),
                hpgNCells: Number(formData.get("hpgNCells")),
                poleCoverageGoal: Number(formData.get("poleCoverageGoal")),
                maxPoles: Number(formData.get("maxPoles")),
                nFFPoles: Number(formData.get("nFFPoles")),
                nFFPiers: Number(formData.get("nFFPiers")),
                polySimplTolerance: Number(formData.get("polySimplTolerance")),
                prngSeed: Number(formData.get("prngSeed")),
                nSamples: Number(formData.get("nSamples")),
                lsFrac: Number(formData.get("lsFrac")),
            };
            setConfig(newConfig);
        }
        closeConfig();
    };

    return (
        <header className={styles.header}>
            <Link to="/" className={styles.logo}>
                <img src="./jaguars_logo.svg" alt="jagua-rs logo" />
                <h1>jagua-rs</h1>
            </Link>
            <FontAwesomeIcon
                icon={faGear}
                size="3x"
                onClick={toggleConfig}
                style={{ color: "#333", cursor: "pointer" }}
            />

            {showSettings && (
                <div className={styles.settingsPanel}>
                    <h2>Settings</h2>
                    <form>
                        <h3>Config of Collision Detection Engine</h3>
                        <label>
                            <input
                                type="number"
                                defaultValue={config.quadtreeDepth}
                                className={styles.input}
                            />
                            Quadtree depth
                        </label>

                        <label>
                            <input
                                type="number"
                                defaultValue={config.hpgNCells}
                                className={styles.input}
                            />
                            #Cells of the hazard proximity grid
                        </label>

                        <h3>Config surrogate item</h3>
                        <label>
                            <input
                                type="number"
                                step="1"
                                defaultValue={config.poleCoverageGoal}
                                className={styles.input}
                                min="0"
                                max="100"
                            />
                            Pole coverage goal
                        </label>

                        <label>
                            <input
                                type="number"
                                defaultValue={config.maxPoles}
                                className={styles.input}
                            />
                            Max #poles
                        </label>

                        <label>
                            <input
                                type="number"
                                defaultValue={config.nFFPoles}
                                className={styles.input}
                            />
                            #Poles for fail-fast collision detection
                        </label>

                        <label>
                            <input
                                type="number"
                                defaultValue={config.nFFPiers}
                                className={styles.input}
                            />
                            #Piers for fail-fast collision detection
                        </label>

                        <hr />

                        <label>
                            <input
                                type="number"
                                step="0.01"
                                defaultValue={config.polySimplTolerance}
                                className={styles.input}
                                min="0"
                                max="100"
                            />
                            Polygon simplify tolerance
                        </label>

                        <label>
                            <input
                                type="number"
                                defaultValue={config.prngSeed}
                                className={styles.input}
                            />
                            PRNG Seed
                        </label>

                        <label>
                            <input
                                type="number"
                                defaultValue={config.nSamples}
                                className={styles.input}
                            />
                            #Samples
                        </label>

                        <label>
                            <input
                                type="number"
                                step="1"
                                defaultValue={config.lsFrac}
                                className={styles.input}
                                min="0"
                                max="100"
                            />
                            Local search fraction
                        </label>

                        <div className={styles.buttons}>
                            <button
                                type="button"
                                className={styles.closeButton}
                                onClick={closeConfig}
                            >
                                Close
                            </button>
                            <button
                                type="button"
                                className={styles.saveButton}
                                onClick={saveConfig}
                            >
                                Save
                            </button>
                        </div>
                    </form>
                </div>
            )}
        </header>
    );
};

export default Header;
