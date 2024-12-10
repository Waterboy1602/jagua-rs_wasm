import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faGear } from "@fortawesome/free-solid-svg-icons";
import { Link } from "react-router-dom";

import styles from "../style/Header.module.css";

const Header = () => {
    const [showSettings, setShowSettings] = useState(false);

    const toggleSettings = () => {
        setShowSettings(!showSettings);
    };

    const settings = {
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
        ls_frac: 0.2,
    };

    return (
        <header className={styles.header}>
            <Link to="/" className={styles.logo}>
                <img src="./jaguars_logo.svg" alt="jagua-rs logo" />
                <h1>jagua-rs</h1>
            </Link>
            <FontAwesomeIcon icon={faGear} size="3x" onClick={toggleSettings} />

            {showSettings && (
                <div className={styles.settingsPanel}>
                    <h2>Settings</h2>
                    <form>
                        <fieldset>
                            <legend>CDE Config</legend>
                            <label>
                                Quadtree Depth:
                                <input
                                    type="number"
                                    defaultValue={
                                        settings.cde_config.quadtree_depth
                                    }
                                />
                            </label>
                            <label>
                                HPG N Cells:
                                <input
                                    type="number"
                                    defaultValue={
                                        settings.cde_config.hpg_n_cells
                                    }
                                />
                            </label>
                            <fieldset>
                                <legend>Item Surrogate Config</legend>
                                <label>
                                    Pole Coverage Goal:
                                    <input
                                        type="number"
                                        step="0.01"
                                        defaultValue={
                                            settings.cde_config
                                                .item_surrogate_config
                                                .pole_coverage_goal
                                        }
                                    />
                                </label>
                                <label>
                                    Max Poles:
                                    <input
                                        type="number"
                                        defaultValue={
                                            settings.cde_config
                                                .item_surrogate_config.max_poles
                                        }
                                    />
                                </label>
                                <label>
                                    N FF Poles:
                                    <input
                                        type="number"
                                        defaultValue={
                                            settings.cde_config
                                                .item_surrogate_config
                                                .n_ff_poles
                                        }
                                    />
                                </label>
                                <label>
                                    N FF Piers:
                                    <input
                                        type="number"
                                        defaultValue={
                                            settings.cde_config
                                                .item_surrogate_config
                                                .n_ff_piers
                                        }
                                    />
                                </label>
                            </fieldset>
                        </fieldset>
                        <label>
                            Poly Simpl Tolerance:
                            <input
                                type="number"
                                step="0.0001"
                                defaultValue={settings.poly_simpl_tolerance}
                            />
                        </label>
                        <label>
                            PRNG Seed:
                            <input
                                type="number"
                                defaultValue={settings.prng_seed}
                            />
                        </label>
                        <label>
                            N Samples:
                            <input
                                type="number"
                                defaultValue={settings.n_samples}
                            />
                        </label>
                        <label>
                            LS Fraction:
                            <input
                                type="number"
                                step="0.01"
                                defaultValue={settings.ls_frac}
                            />
                        </label>
                        <button type="submit">Save</button>
                    </form>
                </div>
            )}
        </header>
    );
};

export default Header;
