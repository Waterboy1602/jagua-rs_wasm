import { useEffect, useRef } from "react";
import * as d3 from "d3";

import SVG from "./../assets/1.svg?react";

import styles from "../styles/SVGMani.module.css";

import init, * as wasm from "../../wasm/pkg/wasm";

const SVGManipulation = () => {
    const svgRef = useRef<SVGSVGElement>(null);

    useEffect(() => {
        const uses = svgRef!.current!.querySelectorAll("g > use");

        uses.forEach((use) => {
            const element = d3.select(use);

            const drag = d3
                .drag()
                .subject(function () {
                    const transform = element.attr("transform");
                    const translateMatch = transform.match(/translate\(([^,]+),\s*([^)]+)\)/);
                    if (translateMatch) {
                        return {
                            x: parseFloat(translateMatch[1]),
                            y: parseFloat(translateMatch[2]),
                        };
                    }
                    return { x: 0, y: 0 };
                    // return { x: path.attr("x"), y: path.attr("y") };
                })
                .on("start", function (event) {
                    const transform = element.attr("transform");
                    const translateMatch = transform
                        ? transform.match(/translate\(([^,]+),\s*([^)]+)\)/)
                        : null;
                    if (translateMatch) {
                        console.log(translateMatch);
                        event.subject.x = parseFloat(translateMatch[1].split(" ")[0]);
                        event.subject.y = parseFloat(translateMatch[1].split(" ")[1]);
                    }
                    console.log(event.subject.x, event.subject.y);

                    wasm.toggle_box();
                })
                .on("drag", function (event) {
                    console.log(event.x, event.y);

                    element.attr("transform", `translate(${event.x}, ${event.y})`);
                })
                .on("end", function () {
                    wasm.toggle_box();
                });

            element.call(drag);
        });

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
