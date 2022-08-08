import React from 'react';
import {createRoot} from 'react-dom/client';
import './index.css';
import GalleryView from './GalleryView';
import ModelView from './ModelView';
import {BrowserRouter as Router, Route, Routes,} from "react-router-dom";
import {createTheme, CssBaseline, ThemeProvider} from "@mui/material";
import {grey} from "@mui/material/colors";

const container = document.getElementById('root');
const root = createRoot(container);
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

function render_gallery() {
    const request = new Request("/api/models", {
        method: 'GET',
        headers: new Headers({
            'Content-Type': 'application/json'
        })
    });

    fetch(request)
        .then(resp => resp.json())
        .then(models => {
            root.render(
                <React.StrictMode>
                    <ThemeProvider theme={prefersDark ? createTheme(dark) : createTheme(light)}>
                        <CssBaseline />
                        <Router>
                            <Routes>
                                <Route exact path="/" element={ <GalleryView models={models}/> } />
                                <Route exact path="/:id" element={ <ModelView models={models}/> } />
                                <Route element={ <GalleryView /> } />
                            </Routes>
                        </Router>
                    </ThemeProvider>
                </React.StrictMode>
            );
        });
}

render_gallery();
