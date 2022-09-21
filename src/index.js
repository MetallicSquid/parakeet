import React, {useEffect, useState} from 'react';
import ReactDOM from 'react-dom';
import {BrowserRouter, Route, Routes, useParams} from 'react-router-dom';
import './index.css';
import GalleryView from './GalleryView';
import {createTheme, CssBaseline, ThemeProvider} from "@mui/material";
import {grey} from "@mui/material/colors";
import ModelView from "./ModelView";

const prefersDark = window.matchMedia("(prefers-color-scheme:dark)").matches;

const light = {
    palette: {
        background: {
            default: grey[100],
        },
        mode: "light"
    }
};

const dark = {
    palette: {
        background: {
            paper: grey[900],
        },
        mode: "dark"
    }
};

const RenderGalleryView = () => {
    const [models, setModels] = useState();
    useEffect(() => {
        const request = new Request("/api/models", {
            method: 'GET',
            headers: new Headers({
                'Content-Type': 'application/json'
            })
        });

        const getModels = async () => {
            setModels(await (await fetch(request)).json());
        }

        getModels();
    }, []);

    return models && <GalleryView models={models}/>

}

const RenderModelView = () => {
    const {id} = useParams();
    const [model, setModel] = useState();

    useEffect(() => {
        const request = new Request("/api/models/" + id, {
            method: 'GET',
            headers: new Headers({
                'Content-Type': 'application/json'
            })
        });

        const getModel = async () => {
            setModel(await (await fetch(request)).json());
        }

        getModel();
    }, [])

    return model && <ModelView model={model}/>
}

ReactDOM.render(
     <React.StrictMode>
        <ThemeProvider theme={prefersDark ? createTheme(dark) : createTheme(light)}>
            <CssBaseline />
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={ <RenderGalleryView /> } />
                    <Route path="/:id" element={ <RenderModelView /> } />
                </Routes>
            </BrowserRouter>
        </ThemeProvider>
    </React.StrictMode>,
    document.getElementById('root')
)
