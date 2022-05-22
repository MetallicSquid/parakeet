import './ModelView.css';
import models from '../src/index.json'

import { useParams } from 'react-router-dom';
import React from 'react';

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


function ModelView() {
    const { id } = useParams();
    const model = GatherModelInfo(id);

    // TODO: Implement the functions and classes below
    // TODO: Make a nicer grid layout
    return(
        <div className="GalleryView-div">
            {Title(model.name, model.author)}
            {/*<Parameters*/}
            {/*    description={model.description}*/}
            {/*    parameters={model.parameters}*/}
            {/*/>*/}
            {/*<Model*/}
            {/*    scad={model.scad_path}*/}
            {/*/>*/}

        </div>
    )
}



export default ModelView;

