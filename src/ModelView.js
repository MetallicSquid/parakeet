import { useParams } from 'react-router-dom';
import { RenderParam } from './ParameterElements';
import {
    RenderSTL,
    CameraControls,
    Axes,
    GridPlane,
} from "./CanvasElements";
import {ButtonDownload, ModelDimensions, TimeSinceUpdate} from "./ModelInfo"
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
    Typography,
    Divider,
    Box,
    List,
    ListItem,
    ListItemText,
    ListItemIcon
} from "@mui/material";
import {Canvas} from "@react-three/fiber";
import {AccessTime, Straighten} from "@mui/icons-material";

function ParamView(props) {
    return (
        <>
            {props.parts.map((part) => {
                return (
                    <div style={{width: "100%"}}>
                        <Typography variant="h6" className="Module-subtitle"><b>{part.name}</b></Typography>
                        <div style={{width: "100%"}}>
                            {part.parameters.map((parameter, index) => {
                                return (
                                    RenderParam(parameter, props.formValues, props.setFormValues, props.onStlChange)
                                );
                            })}
                        </div>
                    </div>
                );
            })}
        </>
    )
}

function genStl(model_id, part_id, formValues, setStl, setDimensions) {
    const url = '/api/generate/' + model_id + '/' + part_id;
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
    console.log(props.model)
    let default_values = {};
    for (let part of props.model.parts) {
        for (let i = 0; i < part.parameters.length; i++) {
            if (part.parameters[i].IntRange) {
                default_values[i] = part.parameters[i].IntRange.default_value;
            } else if (part.parameters[i].FloatRange) {
                default_values[i] = part.parameters[i].FloatRange.default_value;
            } else if (part.parameters[i].StringLength) {
                default_values[i] = part.parameters[i].StringLength.default_value;
            } else if (part.parameters[i].Bool) {
                default_values[i] = part.parameters[i].Bool.default_value;
            } else if (part.parameters[i].IntList) {
                default_values[i] = part.parameters[i].IntList.default_value;
            } else if (part.parameters[i].FloatList) {
                default_values[i] = part.parameters[i].FloatList.default_value;
            } else {
                default_values[i] = part.parameters[i].StringList.default_value;
            }
        }
    }

    const [formValues, setFormValues] = useState(default_values);
    const [committedValues, setCommittedValues] = useState(default_values);

    const [stl, setStl] = useState("");
    const [dimensions, setDimensions] = useState([0.0, 0.0, 0.0])

    const [autoRotate, setAutoRotate] = useState(true);
    const [axes, setAxes] = useState(false);
    const [grid, setGrid] = useState(false);
    const [wireframe, setWireframe] = useState(false);
    const [cameraReset, setCameraReset] = useState(true);

    const [updateTime, setUpdateTime] = useState((new Date()));

    useEffect(() => {
        genStl(props.model.model_id, props.model.parts[0].part_id, committedValues, setStl, setDimensions);
        setUpdateTime((new Date()));
    }, [committedValues])


    const onStlChange = (index, value) => {
        setCommittedValues({
            ...committedValues,
            [index]: value
        })
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
        setCommittedValues(default_values);
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
                <Grid item xs={4} height="100%">
                    <Paper elevation={2} style={{height: "100%", overflow: "auto"}}>
                        <List>
                            <ListItem>
                                <div>
                                    <Typography variant="h3" fontWeight="bold">{props.model.name}</Typography>
                                    <Typography variant="h5">by {props.model.author}</Typography>
                                </div>
                            </ListItem>
                            <Divider />
                            <ListItem>
                                <Typography className="Description-text">{props.model.description}</Typography>
                            </ListItem>
                            <Divider />
                            <ListItem>
                                <ParamView
                                    parts={props.model.parts}
                                    formValues={formValues}
                                    setFormValues={setFormValues}
                                    onStlChange={onStlChange}
                                />
                            </ListItem>
                            <Divider />
                            <ListItem>
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
                            </ListItem>
                            <Divider />
                            <ListItem>
                                <List>
                                    <ListItem>
                                        <ListItemIcon>
                                            <AccessTime />
                                        </ListItemIcon>
                                        <ListItemText>
                                            <TimeSinceUpdate updateTime={updateTime} />
                                        </ListItemText>
                                    </ListItem>
                                    <ListItem>
                                        <ListItemIcon>
                                            <Straighten />
                                        </ListItemIcon>
                                        <ListItemText>
                                            <ModelDimensions dimensions={dimensions} />
                                        </ListItemText>
                                    </ListItem>
                                    <ListItem>
                                        <ButtonDownload stl={stl}/>
                                    </ListItem>
                                </List>
                            </ListItem>
                        </List>
                    </Paper>
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
