import './ModelView.css';
import models from '../src/index.json'

// FIXME: For now, this just points to a static `stl_test.stl` file
import stl_file from './stl_test.stl'

import { useParams } from 'react-router-dom';
import { RenderParam } from './ParameterElements'
import React, {Suspense, useEffect, useRef, useState} from 'react';
import {
    Grid,
    Paper,
    Typography,
    Button,
} from "@mui/material";
import {STLLoader} from "three/examples/jsm/loaders/STLLoader";
import {OrbitControls} from "three/examples/jsm/controls/OrbitControls"
import {Canvas, extend, useFrame, useLoader, useThree} from "@react-three/fiber";

extend({ OrbitControls });

function GatherModelInfo() {
    const { id } = useParams();
    for (let i = 0; i < models.length; i++) {
        if (models[i].id === id) {
            return models[i]
        }
    }
    return {}
}

function Parameters(parameters, id, onSTLChange) {
    const default_values = {};

    for (let i = 0; i < parameters.length; i++) {
        if (parameters[i].default.IntParam) {
            default_values[i] = parameters[i].default.IntParam;
        } else if (parameters[i].default.FloatParam) {
            default_values[i] = parameters[i].default.FloatParam;
        } else if (parameters[i].default.StringParam) {
            default_values[i] = parameters[i].default.StringParam;
        } else {
            default_values[i] = parameters[i].default.BoolParam;
        }
    }

    const [formValues, setFormValues] = useState(default_values);

    const handleSubmit = (event) => {
        event.preventDefault();

        const url = 'api/generate/' + id;
        const request = new Request(url, {
            method: 'POST',
            body: JSON.stringify(formValues),
            headers: new Headers({
                'Content-Type': 'application/json'
            })
        })

        fetch(request).then(res => onSTLChange(res));
    }

    return (
        <form onSubmit={handleSubmit}>
            {parameters.map((parameter, index) => (
                RenderParam(parameter, index, formValues, setFormValues)
            ))}

            <Button
                className="Submit-button"
                variant="outlined"
                type="submit"
                fullWidth
            >
                Submit
            </Button>
        </form>
    )
}

function STL(stl) {
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

function ModelView() {
    const model = GatherModelInfo();

    const CameraControls = () => {
        const { camera, gl: { domElement} } = useThree();
        const controls = useRef();
        useFrame((state) => controls.current.update());
        return <orbitControls ref={controls} args={[camera, domElement]} />;
    }

    const [stl, setStl] = useState(stl_file);

    const onSTLChange = (new_stl) => {
        setStl(new_stl);
    }

    return (
        <div className="GalleryView-div">
            <div className="Model-title-div">
                <h1 className="Title-heading">{model.name}</h1>
                <h3 className="Author-subheading">by {model.author}</h3>
            </div>
            <Grid container spacing={4} justifyContent="center">
                <Grid item xs={4.5}>
                    <Paper elevation={1} className="Parameter-paper">
                        <Typography>{model.description}</Typography>
                    </Paper>
                    <Paper elevation={1} className="Parameter-paper">
                        {Parameters(model.parameters, model.id, onSTLChange)}
                    </Paper>
                </Grid>
                <Grid item xs={6.5}>
                    <Paper elevation={1} className="Parameter-paper">
                        <Canvas camera={{position: [0, 10, 20]}}>
                            <Suspense fallback={null}>
                                <STL stl={stl} />
                            </Suspense>
                            <CameraControls />
                        </Canvas>
                    </Paper>
                </Grid>
            </Grid>
        </div>
    )
}

export default ModelView;
