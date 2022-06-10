import React, {useEffect, useRef} from "react";
import {extend, useFrame, useLoader, useThree} from "@react-three/fiber";
import {STLLoader} from "three/examples/jsm/loaders/STLLoader";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";
import {Checkbox, Typography} from "@mui/material";
import {Autorenew, LineAxis} from "@mui/icons-material";
import {AxesHelper} from "three";

extend({ OrbitControls });

export function RenderSTL(stl) {
    const geometry = useLoader(STLLoader, stl.stl);
    const ref = useRef();
    const { camera } = useThree();
    useEffect(() => {
        camera.lookAt(ref.current.position);
    });

    return (
        <>
            <mesh ref={ref}>
                <primitive object={geometry} attach="geometry" />
                <meshStandardMaterial color={"orange"} />
            </mesh>
            <ambientLight />
            <pointLight position={[10, 10, 10]} />
        </>
    );
}

export function CameraControls(autoRotate) {
    const { camera, gl: { domElement} } = useThree();
    const controls = useRef();
    useFrame((state) => controls.current.update());
    return <orbitControls ref={controls} args={[camera, domElement]} autoRotate={autoRotate.autoRotate}/>;
}

export function Axes(axes) {
    if (axes.axes) {
        return <primitive object={new AxesHelper(20)} />
    }
}

export class TimeSinceUpdate extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            time: 0,
            start: Date.now()
        }
    }

    render() {
        this.timer = setInterval(() => {
            this.setState({
                time: Date.now() - this.state.start
            });
        }, 1000);

        const {time} = this.state;
        const minutes = Math.floor(time / 60000);
        const hours = Math.floor(time / 3600000);

        if (minutes < 60) {
            return (
                <Typography>Updated {minutes} {(minutes === 1) ? "minute" : "minutes"} ago.</Typography>
            )
        } else {
            return (
                <Typography>Updated {hours} {(hours === 1) ? "hour" : "hours"} ago.</Typography>
            )
        }
    }
}

export function CheckAutoRotate(onChange) {
    return (
        <Checkbox defaultChecked={true} onChange={onChange} icon={<Autorenew />} checkedIcon={<Autorenew style={{color: "blue"}} />} />
    )
}

export function CheckAxes(onChange) {
    return (
        <Checkbox defaultChecked={true} onChange={onChange} icon={<LineAxis />} checkedIcon={<LineAxis style={{color: "blue"}} />} />
    )
}
