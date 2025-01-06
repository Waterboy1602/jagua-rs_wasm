import { useEffect, useRef } from "react";
import * as d3 from "d3"; // Import D3 library

import SVG from "./../assets/1.svg?react";

import styles from "../styles/Solution.module.css";

const SVGManipulation = () => {
    const svgRef = useRef<SVGSVGElement>(null);

    useEffect(() => {
        // Select the SVG and path elements after the component mounts
        const paths = svgRef!.current!.querySelectorAll("g > path");

        for (let i = 0; i < paths.length; i++) {
            const path = d3.select(paths[i]);

            const drag = d3
                .drag()
                .subject(function () {
                    return { x: path.attr("x"), y: path.attr("y") };
                })
                .on("start", function () {
                    // This will be triggered when dragging starts
                    console.log("Drag started");
                })
                .on("drag", function (event) {
                    // During dragging, move the element based on the drag's delta (movement)
                    path.attr("transform", `translate(${event.x}, ${event.y})`);
                })
                .on("end", function () {
                    // This will be triggered when the drag ends
                    console.log("Drag ended");
                });

            // Apply drag behavior to the path element
            path.call(drag);
        }
    }, []);

    return (
        <div className={`${styles.container} ${styles.result}`}>
            <h1>SVG Manipulation</h1>

            <div className={`${styles.container} ${styles.solution}`}>
                <SVG width={1000} height={500} ref={svgRef} />
            </div>
        </div>
    );
};

export default SVGManipulation;
