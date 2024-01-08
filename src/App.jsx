import { useState } from "react";
import "./App.css";
import { SketchPicker } from "react-color";
import { invoke } from "@tauri-apps/api";

function App() {
    const [color, setColor] = useState("#5b95b5")

    return <div>
        <SketchPicker
            color={color}
            onChange={(color, evt) => {
                setColor(color);
                console.log(color);
                invoke("generate_gradient", color.rgb);
            }}
        />
    </div>
}

export default App;
