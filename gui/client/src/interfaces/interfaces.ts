export interface Config {
    quadtreeDepth: number;
    hpgNCells: number;
    poleCoverageGoal: number;
    maxPoles: number;
    nFFPoles: number;
    nFFPiers: number;
    polySimplTolerance: number;
    prngSeed: number;
    nSamples: number;
    lsFrac: number;
}

export interface Input {
    name: string;
    items: Item[];
    strip: Strip;
}

export interface Strip {
    Height: number;
}

export interface Shape {
    Type: string;
    Data: number[][];
}

export interface Item {
    Demand: number;
    DemandMax: number;
    AllowedOrientations: number[];
    Shape: Shape;
}
