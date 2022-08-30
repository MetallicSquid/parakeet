import './ModelView.css';

import { useParams } from 'react-router-dom';
import { RenderParam } from './ParameterElements';
import {
    RenderSTL,
    CameraControls,
    Axes,
    GridPlane,
} from "./CanvasElements";
import { TimeSinceUpdate } from "./ModelInfo"
import {
    CheckAutoRotate,
    CheckAxes,
    ButtonResetCamera,
    CheckGrid,
    CheckWireframe,
    ButtonResetParameters
} from "./Toolbar"
import React, {Suspense, useEffect, useState} from 'react';
import {
    Grid,
    Stack,
    Paper,
    Container,
    Typography,
    Divider,
    Box,
} from "@mui/material";
import {Canvas} from "@react-three/fiber";

function GatherModelInfo(models) {
    const { id } = useParams();
    for (let i = 0; i < models.length; i++) {
        if (models[i].id === id) {
            return models[i]
        }
    }
    return {}
}

function ParamView(props) {
    return (
        <>
            {props.modules.map((module) => {
                return (
                    <>
                        <Typography variant="h6" className="Module-subtitle"><b>{module.name}</b></Typography>
                        <div className="Parameter-container">
                            {module.parameters.map((parameter) => {
                                return (
                                    RenderParam(parameter, props.formValues, props.setFormValues, props.onStlChange)
                                );
                            })}
                        </div>
                    </>
                );
            })}
        </>
    )
}

function genStl(id, formValues, setStl, setDimensions) {
    const url = '/api/generate/' + id;
    const request = new Request(url, {
        method: 'POST',
        body: JSON.stringify(formValues),
        headers: new Headers({
            'Content-Type': 'application/json'
        })
    });

    fetch(request)
        .then(resp => resp.json())
        .then(json => {
            setStl(json["filename"]);
            setDimensions(json["dimensions"])
        });
}

function ModelView(props) {
    const model = GatherModelInfo(props.models);

    let default_values = {};
    for (let module of model.modules) {
        for (let i = 0; i < module.parameters.length; i++) {
            if (module.parameters[i].default.IntParam) {
                default_values[i] = module.parameters[i].default.IntParam;
            } else if (module.parameters[i].default.FloatParam) {
                default_values[i] = module.parameters[i].default.FloatParam;
            } else if (module.parameters[i].default.StringParam) {
                default_values[i] = module.parameters[i].default.StringParam;
            } else {
                default_values[i] = module.parameters[i].default.BoolParam;
            }
        }
    }

    // FIXME: I feel like I might be abusing usage of state at this point
    const [prevValues, setPrevValues] = useState({});
    const [formValues, setFormValues] = useState(default_values);

    const [requestStl, setRequestStl] = useState(false);
    const [stl, setStl] = useState("");
    const [newModel, setNewModel] = useState(false);
    const [dimensions, setDimensions] = useState([50.0, 50.0, 50.0])

    const [autoRotate, setAutoRotate] = useState(true);
    const [axes, setAxes] = useState(false);
    const [grid, setGrid] = useState(false);
    const [wireframe, setWireframe] = useState(false);
    const [cameraReset, setCameraReset] = useState(true);

    // FIXME: These two useEffects should be merged into one but timings need to be sorted out
    useEffect(() => {
        genStl(model.id, formValues, setStl, setDimensions);
        setPrevValues(formValues);
    }, [newModel]);

    useEffect(() => {
        if (requestStl) {
            let changed = false;
            for (let key in formValues) {
                if (formValues[key] !== prevValues[key]) {
                    changed = true;
                }
            }

            if (changed) {
                setNewModel(!newModel);
            }
            setRequestStl(false);
        }
    }, [formValues])

    const onStlChange = () => {
        setRequestStl(true);
    }

    const onAutoRotateChange = (event) => {
        setAutoRotate(event.target.checked);
    }

    const onAxesChange = (event) => {
        setAxes(event.target.checked);
    }

    const onGridChange = (event) => {
        setGrid(event.target.checked);
    }

    const onWireframeChange = (event) => {
        setWireframe(event.target.checked);
    }

    const onCameraReset = () => {
        setCameraReset(true);
    }

    const onParametersReset = () => {
        setFormValues(default_values);
        onStlChange();
    }

    return (
        <Box sx={{flexGrow: 1, height: "100vh"}}>
            <Grid
                container
                direction="row"
                justifyContent="center"
                alignItems="stretch"
                spacing={4}
                padding={4}
                height="100%"
            >
                <Grid
                    container
                    item
                    direction="column"
                    justifyContent="space-between"
                    alignItems="stretch"
                    spacing={4}
                    xs={4}
                    height="100%"
                    // FIXME: This behaviour isn't ideal honestly, maybe `minSize` is needed
                    overflow="hidden"
                >
                    <Grid item>
                        <Container>
                            <Typography variant="h2" fontWeight="bold">{model.name}</Typography>
                            <Typography variant="h4">by {model.author}</Typography>
                        </Container>
                    </Grid>

                    <Grid item>
                        <Paper elevation={2}>
                            <Typography className="Description-text">{model.description}</Typography>
                            <Divider />
                            <ParamView
                                modules={model.modules}
                                formValues={formValues}
                                setFormValues={setFormValues}
                                onStlChange={onStlChange}
                            />
                            <Divider />
                            <Stack
                                direction="row"
                                alignItems="center"
                                justifyContent="flex-start"
                                spacing={2}
                                className="Toolbar"
                            >
                                <CheckAutoRotate onChange={onAutoRotateChange} />
                                <Divider orientation="vertical" flexItem />
                                <CheckAxes onChange={onAxesChange} />
                                <CheckGrid onChange={onGridChange} />
                                <CheckWireframe onChange={onWireframeChange} />
                                <Divider orientation="vertical" flexItem />
                                <ButtonResetCamera onClick={onCameraReset} />
                                <ButtonResetParameters onClick={onParametersReset} />
                            </Stack>
                        </Paper>
                    </Grid>

                    <Grid item>
                        <Paper elevation={2} className="Info-container">
                            <TimeSinceUpdate />
                        </Paper>
                    </Grid>
                </Grid>

                <Grid item xs overflow="hidden" height="100%">
                    <Paper elevation={2} sx={{height: "100%"}}>
                        <Canvas camera={{up: [0, 0, 1]}}>
                            <Suspense fallback={null}>
                                <RenderSTL
                                    stl={stl}
                                    dimensions={dimensions}
                                    cameraReset={cameraReset}
                                    setCameraReset={setCameraReset}
                                    wireframe={wireframe}
                                />
                            </Suspense>
                            <CameraControls autoRotate={autoRotate} />
                            <Axes axes={axes} size={Math.max(dimensions[0], dimensions[1], dimensions[2])} />
                            <GridPlane grid={grid} size={Math.max(dimensions[0], dimensions[1], dimensions[2]) * 2} />
                        </Canvas>
                    </Paper>
                </Grid>
            </Grid>
        </Box>
    )
}

export default ModelView;
