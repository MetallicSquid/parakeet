import './ModelView.css';
import models from '../src/index.json'

import { useParams } from 'react-router-dom';
import {
    RenderParam
} from './ParameterElements'
import React, {useState} from 'react';
import {
    Grid,
    Paper,
    Typography,
    Button
} from "@mui/material";

function GatherModelInfo(id) {
    for (let i = 0; i < models.length; i++) {
        if (models[i].id === id) {
            return models[i]
        }
    }
    return {}
}

function Title(name, author) {
    return (
        <div className="Model-title-div">
            <h1 className="Title-heading">{name}</h1>
            <h3 className="Author-subheading">by {author}</h3>
        </div>
    )
}

function Parameters(parameters) {
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

    // TODO: Make use of the data returned from here
    const handleSubmit = (event) => {
        event.preventDefault();
        console.log(formValues);
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

function ModelView() {
    const { id } = useParams();
    const model = GatherModelInfo(id);

    // TODO: Implement the functions and classes below
    return(
        <div className="GalleryView-div">
            {Title(model.name, model.author)}
            <Grid container spacing={4} justifyContent="center">
                <Grid item xs={3.5}>
                    <Paper elevation={1} className="Parameter-paper">
                        <Typography>{model.description}</Typography>
                    </Paper>
                    <Paper elevation={1} className="Parameter-paper">
                        {Parameters(model.parameters)}
                    </Paper>
                </Grid>
                <Grid item xs={5.5}>
                    <Paper elevation={1} className="Parameter-paper">
                        {/*<Model*/}
                        {/*    scad={model.scad_path}*/}
                        {/*/>*/}
                    </Paper>
                </Grid>
            </Grid>
        </div>
    )
}

export default ModelView;

