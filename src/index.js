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

const container = document.getElementById('root');
const root = createRoot(container);

root.render(
    <React.StrictMode>
        <Router>
            <Routes>
                <Route exact path="/" element={ <GalleryView /> } />
                <Route exact path="/:id" element={ <ModelView /> } />
                <Route element={ <GalleryView /> } />
            </Routes>
        </Router>
    </React.StrictMode>
);
