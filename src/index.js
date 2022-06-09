import React from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import GalleryView from './GalleryView';
import ModelView from './ModelView';
import {
    BrowserRouter as Router,
    Routes,
    Route,
} from "react-router-dom";
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
}

const dark = {
    palette: {
        background: {
            paper: grey[900],
        },
        mode: "dark"
    }
}

root.render(
    <React.StrictMode>
        <ThemeProvider theme={prefersDark ? createTheme(dark) : createTheme(light)}>
            <CssBaseline />
            <Router>
                <Routes>
                    <Route exact path="/" element={ <GalleryView /> } />
                    <Route exact path="/:id" element={ <ModelView /> } />
                    <Route element={ <GalleryView /> } />
                </Routes>
            </Router>
        </ThemeProvider>
    </React.StrictMode>
);
