import './ModelView.css';

import { useParams } from 'react-router-dom';
import { RenderParam } from './ParameterElements';
import {
    RenderSTL,
    CameraControls,
    Axes,
    TimeSinceUpdate,
    CheckAutoRotate,
    CheckAxes
} from "./STLElements";
import React, {Suspense, useState} from 'react';
import {
    Grid,
    Paper,
    Typography,
    Button,
    CardActions,
    CardContent,
    Box,
    Stack
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

function ParamView(modules, model_id, formValues, setFormValues, onStlChange) {
    return (
        <CardContent>
            {modules.map((module) => {
                return (
                    <div>
                        <Typography variant="h6" className="Module-title"><b>{module.name}</b></Typography>
                        {module.parameters.map((parameter) => {
                            return (
                                RenderParam(parameter, formValues, setFormValues, onStlChange)
                            );
                        })}
                    </div>
                );
            })}
        </CardContent>
    )
}

function genStl(id, formValues, setStl) {
    console.log("generate");
    const url = '/api/generate/' + id;
    const request = new Request(url, {
        method: 'POST',
        body: JSON.stringify(formValues),
        headers: new Headers({
            'Content-Type': 'application/json'
        })
    });

    fetch(request)
        .then(resp => resp.text())
        .then(text => {
            setStl(text);
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

    const [formValues, setFormValues] = useState(default_values);
    const [stl, setStl] = useState("");
    const [autoRotate, setAutoRotate] = useState(true);
    const [axes, setAxes] = useState(true);

    if (stl === "") {
        genStl(model.id, formValues, setStl);
    }

    const onStlChange = (event) => {
        genStl(model.id, formValues, setStl);
    }

    const onAutoRotateChange = (event) => {
        setAutoRotate(event.target.checked);
    }

    const onAxesChange = (event) => {
        setAxes(event.target.checked);
    }

    return (
        <div className="ModelView-div">
            <div className="Model-title-div">
                <h1 className="Title-heading">{model.name}</h1>
                <h3 className="Author-subheading">by {model.author}</h3>
            </div>
            <Grid container spacing={4} justifyContent="center" style={{height: "86vh"}}>
                <Grid item xs={4.5}>
                    <Paper elevation={1} className="Parameter-paper" style={{height: "10%"}}>
                        <Typography>{model.description}</Typography>
                    </Paper>
                    <Paper elevation={1} className="Parameter-paper" style={{height: "70%"}}>
                        {ParamView(model.modules, model.id, formValues, setFormValues, onStlChange)}
                    </Paper>
                </Grid>
                <Grid item xs={6.5}>
                    <Paper elevation={1} className="Parameter-paper" style={{height: "70%"}}>
                        <Canvas camera={{position: [0, 10, 20]}} style={{height: "90%"}}>
                            <Suspense fallback={null}>
                                <RenderSTL stl={stl} />
                            </Suspense>
                            <Axes axes={axes} />
                            <CameraControls autoRotate={autoRotate} />
                        </Canvas>
                        <Box className="Controls-box">
                            <Stack
                                direction="row"
                                justifyContent="flex-start"
                                alignItems="center"
                                spacing={4}
                            >
                                <TimeSinceUpdate />
                                {CheckAutoRotate(onAutoRotateChange)}
                                {CheckAxes(onAxesChange)}
                            </Stack>
                        </Box>
                    </Paper>
                    <Paper elevation={1} className="Parameter-paper" style={{height: "10%"}}>

                    </Paper>
                </Grid>
            </Grid>
        </div>
    )
}

export default ModelView;
