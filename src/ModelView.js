import './ModelView.css';
import models from '../src/index.json'

import { useParams } from 'react-router-dom';
import {
    IntRange,
    FloatRange,
    StringLength,
    IntList,
    FloatList,
    StringList,
    BoolCheck
} from './ParameterElements'
import React from 'react';
import {
    Grid,
    Paper, Typography
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
    return (
        <div>
            {IntRange(parameters[0])}
            {IntList(parameters[1])}
            {FloatRange(parameters[2])}
            {FloatList(parameters[3])}
            {StringLength(parameters[4])}
            {StringList(parameters[5])}
            {BoolCheck(parameters[6])}

        </div>
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
                <Grid item>
                    <Paper variant="outlined" className="Parameter-paper">
                        <Typography>{model.description}</Typography>
                    </Paper>
                    <Paper variant="outlined" className="Parameter-paper">
                        {Parameters(model.parameters)}
                    </Paper>
                </Grid>

                {/*<Model*/}
                {/*    scad={model.scad_path}*/}
                {/*/>*/}
            </Grid>


        </div>
    )
}



export default ModelView;

