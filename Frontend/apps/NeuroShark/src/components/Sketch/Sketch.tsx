import * as React from "react";
import { P5CanvasInstance, ReactP5Wrapper } from "@p5-wrapper/react";
import Eraser1 from '@/assets/images/eraser1.png';
import SaveImage from '@/assets/images/diskette.png';
import './Sketch.scss';
import { useState } from "react";

type SketchProps = {
    onSaveDataOfSketch: any
};

const toRest = 140;
const len = 784;

export function Sketch({ onSaveDataOfSketch }: SketchProps) {
    const [eraseSketch, setEraseSketch] = useState(false);
    const [getData, setGetData] = useState(false);
    const [data, setdata] = useState<any>([]);
    const test = React.useRef<number[]>([]);

    const setGetDataToFalse = () => {
        setGetData(false);
    }

    function sketch(p5: P5CanvasInstance) {
        var inputs: any = [];
        let read_data = false;


        p5.setup = () => {
            p5.createCanvas(280, 280, p5.WEBGL);
            p5.background(250);
        };
    
        p5.draw = () => {
            p5.strokeWeight(8);
            p5.stroke(0);

            if (p5.mouseIsPressed) {
                p5.line(p5.pmouseX - toRest, p5.pmouseY - toRest, p5.mouseX - toRest, p5.mouseY - toRest);
            }

            if (eraseSketch) {
                p5.background(250);
                setEraseSketch(false);
                console.log("Sketch clear!");
            }

            // inputs = [];
            test.current = [];
            const img = p5.get();
            img.resize(28, 28);
            img.loadPixels();
            // console.log(img);
            
            for (let i = 0; i < len; i++) {
                let bright = img.pixels[i * 4];
                bright += bright == 250 ? 5 : 0;
                // inputs[i] = (255 - bright) / 255.0;
                test.current[i] = (255 - bright) / 255.0;
            }
        }
    }

    return (
        <div className="sketch__container">
            <ReactP5Wrapper sketch={sketch} />
            <button className="sketch_eraser-button" onClick={() => setEraseSketch(true)}>
                <img src={Eraser1} alt="eraser" className="sketch_eraser-button-image"/>
            </button>
            <button className="sketch_eraser-button sketch_eraser--left" onClick={() => {
                console.log("INFORMATION SAVING... ");
                if (onSaveDataOfSketch)
                    onSaveDataOfSketch(test.current.slice());
                console.log("SAVED!");
                
            }}>
                <img src={SaveImage} alt="eraser" className="sketch_eraser-button-image"/>
            </button>
        </div>
    );
}

