import React, {useEffect, useRef} from "react";
import {extend, useFrame, useLoader, useThree} from "@react-three/fiber";
import {STLLoader} from "three/examples/jsm/loaders/STLLoader";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";
import {Checkbox, Tooltip, Typography} from "@mui/material";
import {Autorenew, LineAxis} from "@mui/icons-material";
import {AxesHelper} from "three";

extend({ OrbitControls });

export function RenderSTL(stl) {
    console.log("render")
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
        const date_hours = (new Date(Date.now()).getHours()).toString();
        const date_minutes = (new Date(Date.now()).getMinutes()).toString();
        this.state = {
            start: Date.now(),
            hours: date_hours.length === 1 ? "0" + date_hours : date_hours,
            minutes: date_minutes.length === 1 ? "0" + date_minutes : date_minutes,
            date: new Date(Date.now()).toDateString(),
            duration_string: "0 seconds ago."
        }
    }

    render() {
        const calculateDurationString = () => {
            const time = Date.now() - this.state.start;
            const seconds = Math.floor(time / 1000);
            const minutes = Math.floor(time / 60000);
            const hours = Math.floor(time / 3600000);

            if (seconds < 60) {
                this.setState({ duration_string: (seconds + (seconds === 1 ? " second " : " seconds ") + "ago.") });
            } else if (minutes < 60) {
                this.setState({ duration_string: (minutes + (minutes === 1 ? " minute " : " minutes ") + "ago.") });
            } else if (hours < 24) {
                this.setState({ duration_string: (hours + (hours === 1 ? " hour " : " hours ") + "ago.") });
            } else {
                this.setState({ duration_string: "More than a day ago." });
            }
        }

        return (
            <Tooltip title={this.state.duration_string} onOpen={calculateDurationString}>
                <Typography>Last updated at {this.state.hours}:{this.state.minutes} on {this.state.date}.</Typography>
            </Tooltip>
        )
    }
}

export function CheckAutoRotate(onChange) {
    return (
        <Tooltip title={"Toggle Rotation"}>
            <Checkbox defaultChecked={true} onChange={onChange} icon={<Autorenew />} checkedIcon={<Autorenew color="primary" />} />
        </Tooltip>
    )
}

export function CheckAxes(onChange) {
    return (
        <Tooltip title={"Toggle Axes"}>
            <Checkbox defaultChecked={true} onChange={onChange} icon={<LineAxis />} checkedIcon={<LineAxis color="primary" />} />
        </Tooltip>
    )
}
