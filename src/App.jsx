import { useState } from "react";
import "./App.css";
import { SketchPicker } from "react-color";
import { invoke } from "@tauri-apps/api";

function App() {
    const [color, setColor] = useState("#5b95b5")
    const [gradient, setGradient] = useState([])

    return <div>
        <SketchPicker
            color={color}
            onChange={(color, evt) => {
                setColor(color);
                console.log(color);
                invoke("generate_gradient", color.rgb).then((res) => {
                    console.log("res: " + res);
                    setGradient(res);
                });
            }}
        />
        <div className="gradient">
            {gradient.map((color) => {
                return <div style={{
                    padding: "1rem",
                    backgroundColor: `rgb(${color[0]}, ${color[1]}, ${color[2]})`
                }}>
                    rgb({color[0]}, {color[1]}, {color[2]})
                </div>
            })}
        </div>
    </div>
}
export default App;
