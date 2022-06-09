import React, {useEffect, useRef} from "react";
import {extend, useFrame, useLoader, useThree} from "@react-three/fiber";
import {STLLoader} from "three/examples/jsm/loaders/STLLoader";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls";

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

export function CameraControls() {
    const { camera, gl: { domElement} } = useThree();
    const controls = useRef();
    useFrame((state) => controls.current.update());
    return <orbitControls ref={controls} args={[camera, domElement]} autoRotate={true}/>;
}
