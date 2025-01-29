import { useEffect, useRef } from "react";
import * as d3 from "d3"; // Import D3 library

import SVG from "./../assets/1.svg?react";

import styles from "../styles/SVGMani.module.css";

import init, * as wasm from "../../wasm/pkg/wasm";

const SVGManipulation = () => {
    const svgRef = useRef<SVGSVGElement>(null);

    useEffect(() => {
        // Select the SVG and path elements after the component mounts
        const paths = svgRef!.current!.querySelectorAll("g > path");
        console.log(paths);

        for (let i = 1; i < paths.length; i++) {
            const path = d3.select(paths[i]);
            let initialX = 0;
            let initialY = 0;

            const drag = d3
                .drag()
                .subject(function () {
                    return { x: path.attr("x"), y: path.attr("y") };
                })
                .on("start", function () {
                    // This will be triggered when dragging starts
                    console.log("Drag started");
                    const transform = path.attr("transform");
                    if (transform) {
                        const translate = transform.match(/translate\(([^,]+),\s*([^)]+)\)/);
                        if (translate) {
                            initialX = parseFloat(translate[1]);
                            initialY = parseFloat(translate[2]);
                        }
                    }
                })
                .on("drag", function (event) {
                    // During dragging, move the element based on the drag's delta (movement)
                    const newX = initialX + event.x;
                    const newY = initialY + event.y;
                    path.attr("transform", `translate(${newX}, ${newY})`);
                    console.log("Dragging", event.x, event.y);
                })
                .on("end", function () {
                    // This will be triggered when the drag ends
                    console.log("Drag ended");
                    console.log(path.attr("transform"));
                    wasm.toggleBox();
                });

            // Apply drag behavior to the path element
            path.call(drag);
        }

        init()
            .then(() => {
                console.log("WASM Module Loaded:", Object.keys(wasm));

                if (wasm.run) {
                    wasm.run(); // Call the exported `run` function
                } else {
                    console.error("run() is undefined! Check module exports.");
                }
            })
            .catch(console.error);
    }, []);

    return (
        <div>
            <div className={`${styles.container} ${styles.result}`}>
                <h1>SVG Manipulation</h1>
                <div id="testBox" className={`${styles.testBox}`}></div>

                <div className={`${styles.container} ${styles.solution}`}>
                    <SVG width={1000} height={500} ref={svgRef} />
                </div>
            </div>
        </div>
    );
};

export default SVGManipulation;
