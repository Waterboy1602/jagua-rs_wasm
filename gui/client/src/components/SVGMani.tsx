import { useEffect, useRef } from "react";
import * as d3 from "d3";

import SVG from "./../assets/1.svg?react";

import styles from "../styles/SVGMani.module.css";

import init, * as wasm from "../../wasm/pkg/wasm";

const SVGManipulation = () => {
    const svgRef = useRef<SVGSVGElement>(null);

    function roundFloat(num: number) {
        return Number(num.toFixed(3));
    }

    function translatePoint(x: number, y: number, tx: number, ty: number) {
        return [roundFloat(x + tx), roundFloat(y + ty)];
    }

    useEffect(() => {
        const paths = svgRef!.current!.querySelectorAll("g > path");

        for (let i = 1; i < paths.length; i++) {
            const path = d3.select(paths[i]);
            let transX = 0;
            let transY = 0;

            const drag = d3
                .drag()
                .subject(function () {
                    return { x: path.attr("x"), y: path.attr("y") };
                })
                .on("start", function () {
                    wasm.toggle_box();
                })
                .on("drag", function (event) {
                    transX = event.x;
                    transY = event.y;
                    path.attr("transform", `translate(${event.x}, ${event.y})`);
                })
                .on("end", function () {
                    const commands = path.attr("d").split(/(?=[A-Za-z])/);
                    let newDAttribute = "";

                    commands.forEach((command) => {
                        if (command !== "z") {
                            const type = command[0];
                            const coords = command.slice(1).trim().split(",").map(Number);

                            const newCoords = [];
                            for (let i = 0; i < coords.length; i += 2) {
                                const [x, y] = translatePoint(
                                    coords[i],
                                    coords[i + 1],
                                    transX,
                                    transY
                                );
                                newCoords.push(x, y);
                            }

                            newDAttribute += `${type}${newCoords.join(",")} `;
                        } else {
                            newDAttribute += "z";
                        }
                    });

                    path.attr("d", `${newDAttribute.trim()}`);
                    path.attr("transform", `translate(0,0)`);

                    wasm.toggle_box();
                });

            path.call(drag);
        }

        init()
            .then(() => {
                console.log("WASM Module Loaded:", Object.keys(wasm));
            })
            .catch(console.error);
    }, []);

    return (
        <div>
            <div className={`${styles.rust}`}>
                <h1>SVG Manipulation</h1>
                <div id="testBox" className={`green`}></div>
            </div>

            <div className={`${styles.svg}`}>
                <SVG width="100%" height="100%" ref={svgRef} />
            </div>
        </div>
    );
};

export default SVGManipulation;
