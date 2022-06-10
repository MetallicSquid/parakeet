import './ModelView.css';
import models from '../src/index.json'

// FIXME: For now, this just points to a static `stl_test.stl` file
import stl_file from './stl_test.stl'

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
            <CardContent>
                {parameters.map((parameter, index) => (
                    RenderParam(parameter, index, formValues, setFormValues)
                ))}
            </CardContent>
            <CardActions>
                <Button
                    className="Submit-button"
                    variant="outlined"
                    type="submit"
                    fullWidth
                >
                    Submit
                </Button>
            </CardActions>
        </form>
    )
}

function ModelView() {
    const model = GatherModelInfo();

    const [stl, setStl] = useState(stl_file);
    const [autoRotate, setAutoRotate] = useState(true);
    const [axes, setAxes] = useState(true);

    const onSTLChange = (new_stl) => {
        setStl(new_stl);
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
                        {Parameters(model.parameters, model.id, onSTLChange)}
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
