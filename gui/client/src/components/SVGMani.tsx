import { useEffect, useRef } from "react";
import * as d3 from "d3";

import SVG from "./../assets/1.svg?react";

import styles from "../styles/SVGMani.module.css";

import init, * as wasm from "../../wasm/pkg/wasm";

const SVGManipulation = () => {
    const svgRef = useRef<SVGSVGElement>(null);
    const isRKeyPressedRef = useRef(false);

    useEffect(() => {
        const handleKeyDown = (event: KeyboardEvent) => {
            if (event.key === "r" || event.key === "R") {
                isRKeyPressedRef.current = true;
            }
        };

        const handleKeyUp = (event: KeyboardEvent) => {
            if (event.key === "r" || event.key === "R") {
                isRKeyPressedRef.current = false;
            }
        };

        window.addEventListener("keydown", handleKeyDown);
        window.addEventListener("keyup", handleKeyUp);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
            window.removeEventListener("keyup", handleKeyUp);
        };
    }, []);

    useEffect(() => {
        const svgElement = svgRef.current;
        if (!svgElement) return;

        const uses = svgElement.querySelectorAll("g > use");

        uses.forEach((use) => {
            const element = d3.select(use);

            const titleElement = use.querySelector("title");
            if (titleElement) {
                const titleText = titleElement.textContent;
                console.log("Title Text:", titleText);
            }

            let transform = null;
            let translateMatch: number[] = [];
            let isElementSelected = false;

            const elementSelected = (event: d3.D3DragEvent<SVGUseElement, unknown, unknown>) => {
                const rotateElement = () => {
                    if (isElementSelected && isRKeyPressedRef.current) {
                        transform = element.attr("transform");
                        translateMatch = transform.match(/[-+]?\d*\.?\d+/g)?.map(Number) || [];
                        if (translateMatch) {
                            translateMatch[2] = (translateMatch[2] + 1) % 360; // Increment rotation
                            element.attr(
                                "transform",
                                `translate(${event.x}, ${event.y}) rotate(${translateMatch[2]})`
                            );
                        }

                        requestAnimationFrame(rotateElement);
                    }
                };

                rotateElement();
            };

            const drag = d3
                .drag()
                .subject(function () {
                    transform = element.attr("transform");
                    translateMatch = transform.match(/[-+]?\d*\.?\d+/g)?.map(Number) || [];
                    if (translateMatch) {
                        return {
                            x: translateMatch[0],
                            y: translateMatch[1],
                        };
                    }
                    return { x: 0, y: 0 };
                })
                .on("start", function (event) {
                    isElementSelected = true;
                    elementSelected(event);
                    wasm.toggle_box();
                })
                .on("drag", function (event) {
                    element.attr(
                        "transform",
                        `translate(${event.x}, ${event.y}), rotate(${translateMatch[2]})`
                    );
                })
                .on("end", function () {
                    isElementSelected = false;
                    wasm.toggle_box();
                });

            element.call(drag);
        });

        init()
            .then(() => {})
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
