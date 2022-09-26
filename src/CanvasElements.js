import React, {useEffect, useRef} from "react";
import {extend, useFrame, useLoader, useThree} from "@react-three/fiber";
import {STLLoader} from "three/examples/jsm/loaders/STLLoader";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";
import {AxesHelper, GridHelper} from 'three'

extend({ OrbitControls });

export function RenderSTL(props) {
    const geometry = useLoader(STLLoader, props.stl);
    const ref = useRef();

    const { camera } = useThree();
    useEffect(() => {
        camera.lookAt(ref.current.position);

        if (props.cameraReset) {
            let offset = Math.sqrt(Math.pow(props.dimensions[0], 2) + Math.pow(props.dimensions[1], 2) + Math.pow(props.dimensions[2], 2))
            camera.position.set(props.dimensions[0] + (offset/4), props.dimensions[1] + (offset/4), props.dimensions[2] + (offset/4));
            camera.updateProjectionMatrix();
            props.setCameraReset(false);
        }
    });

    return (
        <>
            <mesh ref={ref}>
                <primitive object={geometry} attach="geometry"/>
                <meshStandardMaterial color="orange" wireframe={props.wireframe}/>
            </mesh>
            <ambientLight />
            <pointLight position={props.dimensions}/>
        </>
    );
}

export function CameraControls(props) {
    const { camera, gl: { domElement} } = useThree();
    const controls = useRef();
    useFrame((state) => controls.current.update());
    return <orbitControls
        ref={controls}
        args={[camera, domElement]}
        autoRotate={props.autoRotate}
    />;
}

export function Axes(props) {
    if (props.axes) {
        const rotatedAxesHelper = new AxesHelper(props.size);
        rotatedAxesHelper.rotation.x = Math.PI / 2;
        return <primitive object={rotatedAxesHelper} />
    }
}

export function GridPlane(props) {
    if (props.grid) {
        const rotatedGridHelper = new GridHelper(props.size, 10);
        rotatedGridHelper.rotation.x = Math.PI / 2;
        return <primitive object={rotatedGridHelper} />
    }
}
